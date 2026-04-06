//! Hyle Viewer — 3D Conway's Game of Life using the hyle-ca engine.
//!
//! `cargo run --release -p hyle-viewer`
//!
//! Controls:
//!   Right-drag  — orbit camera
//!   Middle-drag — pan
//!   Scroll      — zoom
//!   WASD / QE   — move camera target
//!   R           — reset simulation
//!   Tab         — toggle mouse capture

mod app;
mod camera;
mod gpu;
mod input;
mod simulation;
mod ui;
mod viewport;
mod world;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_title("Hyle — 3D Conway's Game of Life"),
        ..Default::default()
    };

    eframe::run_native(
        "Hyle Viewer",
        options,
        Box::new(|cc| Ok(Box::new(app::ViewerApp::new(cc)))),
    )
}
