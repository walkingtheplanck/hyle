//! Hyle Viewer — CPU raytracing debug visualizer.
//!
//! `cargo run -p hyle-viewer`
//!
//! Controls:
//!   Left-drag   — orbit camera
//!   Scroll      — zoom in/out
//!   Space       — step simulation (gravity)
//!   R           — reset scene
//!   Esc         — quit

mod camera;
mod raycast;
mod shade;
mod world;

use std::collections::HashSet;
use std::time::Instant;

use glam::IVec3;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rayon::prelude::*;

use hyle_ca::gravity_step;

use camera::Camera;
use world::demo_scene;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const MAX_RAY_STEPS: u32 = 128;

fn main() {
    let (mut world, materials) = demo_scene();

    let mut camera = Camera::new(
        glam::Vec3::new(0.0, 4.0, 0.0), // target: centre of scene, slightly above floor
        30.0,                             // distance
    );

    let mut window = Window::new(
        "Hyle Viewer — CPU Raytracer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .expect("failed to create window");

    // ~60 fps cap
    window.set_target_fps(60);

    let mut buf = vec![0u32; WIDTH * HEIGHT];
    let mut prev_mouse: Option<(f32, f32)> = None;
    let mut frame = 0u64;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let t0 = Instant::now();

        let (w, h) = window.get_size();
        if w * h != buf.len() {
            buf.resize(w * h, 0);
        }

        // -- Input --------------------------------------------------------

        // Orbit
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Pass) {
                if let Some((px, py)) = prev_mouse {
                    camera.rotate(mx - px, -(my - py));
                }
                prev_mouse = Some((mx, my));
            }
        } else {
            prev_mouse = None;
        }

        // Zoom
        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            camera.zoom(scroll_y);
        }

        // Simulation step
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            let mut dirty = HashSet::<IVec3>::new();
            gravity_step(&mut world, &materials, &mut dirty);
        }

        // Reset
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            let (new_world, _) = demo_scene();
            world = new_world;
        }

        // -- Render -------------------------------------------------------

        let eye = camera.eye();
        let ww = w as u32;
        let hh = h as u32;

        buf.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let px = (i % w) as u32;
            let py = (i / w) as u32;
            let dir = camera.ray_dir(px, py, ww, hh);

            *pixel = match raycast::cast_ray(&world, eye, dir, MAX_RAY_STEPS) {
                Some(ref hit) => shade::shade(hit, &materials),
                None => shade::sky_color(),
            };
        });

        window.update_with_buffer(&buf, w, h).unwrap();

        frame += 1;
        if frame % 60 == 0 {
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            eprintln!("frame {frame}: {ms:.1}ms ({w}x{h})");
        }
    }
}
