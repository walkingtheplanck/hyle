//! Hyle Viewer — CPU raytracing debug visualizer with egui UI.
//!
//! `cargo run --release -p hyle-viewer`
//!
//! Controls:
//!   Tab         — toggle mouse capture (orbit without clicking)
//!   Right-drag  — orbit camera
//!   Middle-drag — pan camera
//!   WASD        — pan camera
//!   QE          — move target up/down
//!   Scroll      — zoom in/out
//!   Space       — step simulation (gravity)
//!   R           — reset scene
//!   Esc         — uncapture / quit

mod app;
mod camera;
mod raycast;
mod shade;
mod tools;
mod ui;
mod world;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_title("Hyle Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "Hyle Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(app::ViewerApp::new()))),
    )
}
