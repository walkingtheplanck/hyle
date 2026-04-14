//! Main application state and eframe::App implementation.

use std::collections::VecDeque;
use std::time::Instant;

use eframe::egui;
use glam::Vec3;
use hyle_ca_interface::CaSolverProvider;

use crate::ca::{gol_world, LifeCell, Materials, SimpleWorld, Simulation};
use crate::input::InputState;
use crate::rendering::{draw_toolbar, render, Camera, GpuRaytracer};

pub struct ViewerApp<P>
where
    P: CaSolverProvider<LifeCell>,
{
    world: SimpleWorld,
    materials: Materials,
    camera: Camera,
    gpu: GpuRaytracer,
    sim: Simulation<P>,
    input: InputState,
    world_dirty: bool,
    frame_times: VecDeque<f64>,
    last_frame: Instant,
}

impl<P> ViewerApp<P>
where
    P: CaSolverProvider<LifeCell>,
{
    pub fn new(cc: &eframe::CreationContext, provider: P) -> Self {
        let render_state = cc
            .wgpu_render_state
            .as_ref()
            .expect("wgpu backend required");
        let device = &render_state.device;
        let queue = &render_state.queue;
        let mut renderer = render_state.renderer.write();

        let (mut world, materials) = gol_world();
        let mut sim = Simulation::new(provider);
        sim.reset(&mut world); // prime initial state

        let gpu = GpuRaytracer::new(device, queue, &mut renderer, &world, &materials);
        drop(renderer);

        // Look at the center of the 64³ world.
        let camera = Camera::new(Vec3::new(32.0, 32.0, 32.0), 120.0);

        Self {
            world,
            materials,
            camera,
            gpu,
            sim,
            input: InputState::new(),
            world_dirty: true,
            frame_times: VecDeque::with_capacity(60),
            last_frame: Instant::now(),
        }
    }

    fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg = self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64;
        if avg > 0.0 {
            1000.0 / avg
        } else {
            0.0
        }
    }
}

impl<P> eframe::App for ViewerApp<P>
where
    P: CaSolverProvider<LifeCell>,
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let dt_ms = self.last_frame.elapsed().as_secs_f64() * 1000.0;
        self.last_frame = Instant::now();
        self.frame_times.push_back(dt_ms);
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }

        ctx.request_repaint();

        if self.sim.maybe_auto_step(&mut self.world) {
            self.world_dirty = true;
        }

        let actions = self.input.handle_keyboard(ctx, &mut self.camera);
        if actions.reset {
            self.sim.reset(&mut self.world);
            self.world_dirty = true;
        }

        let vp_size = ctx.available_rect().size();
        let approx_vp = (
            vp_size.x.max(64.0) as u32,
            (vp_size.y - 36.0).max(64.0) as u32,
        );
        let fps = self.fps();

        let (step, reset) = draw_toolbar(
            ctx,
            &mut self.sim.auto_step,
            &mut self.sim.step_interval_ms,
            fps,
            approx_vp,
        );

        if step {
            self.sim.step(&mut self.world);
            self.world_dirty = true;
        }
        if reset {
            self.sim.reset(&mut self.world);
            self.world_dirty = true;
        }

        let render_state = frame
            .wgpu_render_state()
            .expect("wgpu render state")
            .clone();
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                render(
                    ui,
                    &render_state,
                    &mut self.gpu,
                    &self.world,
                    &self.materials,
                    &mut self.camera,
                    &mut self.world_dirty,
                    &mut self.input,
                );
            });
    }
}
