//! Main application state and eframe::App implementation.

use std::collections::{HashSet, VecDeque};
use std::time::Instant;

use eframe::egui::{self, Color32, ColorImage, Sense, TextureHandle, TextureOptions, Vec2};
use glam::{IVec3, Vec3};
use rayon::prelude::*;

use hyle_ca::gravity_step;
use hyle_core::voxel::MaterialId;
use hyle_core::MaterialAccess;

use crate::camera::Camera;
use crate::raycast;
use crate::shade;
use crate::tools::{self, HoverInfo, Tool};
use crate::ui;
use crate::world::{self, Materials, SimpleWorld};

const MAX_RAY_STEPS: u32 = 80;

pub struct ViewerApp {
    world: SimpleWorld,
    materials: Materials,
    camera: Camera,

    // UI state
    tool: Tool,
    selected_material: MaterialId,
    hover_info: Option<HoverInfo>,
    #[allow(dead_code)]
    reset_requested: bool,

    // Rendering
    texture: Option<TextureHandle>,
    pixel_buf: Vec<Color32>,

    // Input
    #[allow(dead_code)]
    prev_drag_pos: Option<egui::Pos2>,
    captured: bool,
    prev_capture_pos: Option<egui::Pos2>,

    // Perf
    frame_times: VecDeque<f64>,
    last_frame: Instant,
}

impl ViewerApp {
    pub fn new() -> Self {
        let (world, materials) = world::demo_scene();
        Self {
            world,
            materials,
            camera: Camera::new(Vec3::new(0.0, 4.0, 0.0), 30.0),
            tool: Tool::Place,
            selected_material: 1, // stone
            hover_info: None,
            reset_requested: false,
            texture: None,
            pixel_buf: Vec::new(),
            prev_drag_pos: None,
            captured: false,
            prev_capture_pos: None,
            frame_times: VecDeque::with_capacity(120),
            last_frame: Instant::now(),
        }
    }

    fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg_ms: f64 = self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64;
        if avg_ms > 0.0 { 1000.0 / avg_ms } else { 0.0 }
    }

    fn render_viewport(&mut self, w: u32, h: u32) {
        let total = (w * h) as usize;
        self.pixel_buf.resize(total, Color32::BLACK);

        let cam_frame = self.camera.frame(w, h);
        let eye = cam_frame.eye;
        let aabb = &self.world.bounds;
        let sky = shade::SKY;
        let world = &self.world;
        let materials = &self.materials;

        self.pixel_buf
            .par_chunks_mut(w as usize)
            .enumerate()
            .for_each(|(py, row)| {
                for (px, pixel) in row.iter_mut().enumerate() {
                    let dir = cam_frame.ray_dir(px as u32, py as u32);
                    *pixel = match raycast::cast_ray(world, eye, dir, aabb, MAX_RAY_STEPS) {
                        Some(ref hit) => shade::shade(hit, materials),
                        None => sky,
                    };
                }
            });
    }

    fn raycast_at_pixel(&self, px: f32, py: f32, w: u32, h: u32) -> Option<raycast::Hit> {
        let cam_frame = self.camera.frame(w, h);
        let dir = cam_frame.ray_dir(px as u32, py as u32);
        raycast::cast_ray(&self.world, cam_frame.eye, dir, &self.world.bounds, MAX_RAY_STEPS)
    }
}

impl eframe::App for ViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Frame timing
        let now = Instant::now();
        let dt_ms = self.last_frame.elapsed().as_secs_f64() * 1000.0;
        self.last_frame = now;
        self.frame_times.push_back(dt_ms);
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }

        // Request continuous repaint for live viewport
        ctx.request_repaint();

        // -- Keyboard shortcuts (global) ----------------------------------

        let input = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::Tab),
                i.key_pressed(egui::Key::Escape),
                i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::R),
                i.key_down(egui::Key::W),
                i.key_down(egui::Key::A),
                i.key_down(egui::Key::S),
                i.key_down(egui::Key::D),
                i.key_down(egui::Key::Q),
                i.key_down(egui::Key::E),
                i.smooth_scroll_delta.y,
            )
        });

        let (tab, esc, space, r_key, w_key, a_key, s_key, d_key, q_key, e_key, scroll_y) = input;

        if tab {
            self.captured = !self.captured;
            self.prev_capture_pos = None;
        }
        if esc {
            if self.captured {
                self.captured = false;
                self.prev_capture_pos = None;
            }
        }
        if space {
            let mut dirty = HashSet::<IVec3>::new();
            gravity_step(&mut self.world, &self.materials, &mut dirty);
        }
        if r_key {
            let (new_world, _) = world::demo_scene();
            self.world = new_world;
        }

        // WASD pan
        let pan_speed = 0.3;
        if w_key { self.camera.target += self.camera.flat_forward() * pan_speed; }
        if s_key { self.camera.target -= self.camera.flat_forward() * pan_speed; }
        if a_key { self.camera.target -= self.camera.flat_right() * pan_speed; }
        if d_key { self.camera.target += self.camera.flat_right() * pan_speed; }
        if q_key { self.camera.target.y -= pan_speed; }
        if e_key { self.camera.target.y += pan_speed; }

        // Scroll zoom (global)
        if scroll_y.abs() > 0.1 {
            self.camera.zoom(scroll_y / 30.0);
        }

        // -- Panels -------------------------------------------------------

        // Compute fps before borrowing self mutably
        let fps = self.fps();

        // Guess viewport size from last frame for display purposes
        let vp_size = ctx.available_rect().size();
        let approx_vp = ((vp_size.x - 180.0).max(64.0) as u32, (vp_size.y - 60.0).max(64.0) as u32);

        let step_sim = ui::draw_top_bar(ctx, &mut self.tool, fps, approx_vp);
        if step_sim {
            let mut dirty = HashSet::<IVec3>::new();
            gravity_step(&mut self.world, &self.materials, &mut dirty);
        }

        ui::draw_side_panel(ctx, &self.materials, &mut self.selected_material);

        let selected_name = self.materials.get_material(self.selected_material).name.clone();
        ui::draw_status_bar(ctx, self.tool, &selected_name, &self.hover_info);

        // -- Central panel (viewport) -------------------------------------

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let avail = ui.available_size();
                let viewport_w = avail.x.max(1.0) as u32;
                let viewport_h = avail.y.max(1.0) as u32;

                // Render
                self.render_viewport(viewport_w, viewport_h);

                // Upload to texture
                let image = ColorImage {
                    size: [viewport_w as usize, viewport_h as usize],
                    source_size: Vec2::new(viewport_w as f32, viewport_h as f32),
                    pixels: self.pixel_buf.clone(),
                };

                let texture = self.texture.get_or_insert_with(|| {
                    ctx.load_texture("viewport", image.clone(), TextureOptions::NEAREST)
                });
                texture.set(image, TextureOptions::NEAREST);

                // Display image with interaction
                let response = ui.add(
                    egui::Image::new(&*texture)
                        .fit_to_exact_size(avail)
                        .sense(Sense::click_and_drag()),
                );

                // -- Mouse interaction on viewport ----------------------------

                // Captured mode: any mouse movement orbits
                if self.captured {
                    if let Some(pos) = response.hover_pos() {
                        if let Some(prev) = self.prev_capture_pos {
                            let dx = pos.x - prev.x;
                            let dy = pos.y - prev.y;
                            if dx.abs() > 0.01 || dy.abs() > 0.01 {
                                self.camera.rotate(dx, dy);
                            }
                        }
                        self.prev_capture_pos = Some(pos);
                    }
                } else {
                    self.prev_capture_pos = None;

                    // Right-drag to orbit
                    if response.dragged_by(egui::PointerButton::Secondary) {
                        let delta = response.drag_delta();
                        self.camera.rotate(delta.x, delta.y);
                    }

                    // Middle-drag to pan
                    if response.dragged_by(egui::PointerButton::Middle) {
                        let delta = response.drag_delta();
                        self.camera.pan(-delta.x, delta.y);
                    }
                }

                // Hover info (single ray under cursor)
                self.hover_info = None;
                if let Some(pos) = response.hover_pos() {
                    let local = pos - response.rect.min;
                    if let Some(hit) = self.raycast_at_pixel(
                        local.x, local.y, viewport_w, viewport_h,
                    ) {
                        self.hover_info =
                            Some(HoverInfo::from_hit(&hit, &self.materials));
                    }
                }

                // Left-click: apply tool
                if response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let local = pos - response.rect.min;
                        if let Some(hit) = self.raycast_at_pixel(
                            local.x, local.y, viewport_w, viewport_h,
                        ) {
                            tools::apply_tool(
                                self.tool,
                                &hit,
                                &mut self.world,
                                self.selected_material,
                            );
                        }
                    }
                }
            });
    }
}
