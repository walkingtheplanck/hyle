//! Hyle Viewer — multi-scenario 3D cellular automata using the hyle-ca framework.
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
mod ca;
mod input;
mod rendering;

use eframe::egui;
use hyle_ca_solver::CpuSolverProvider;
use std::io::Error;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_title("Hyle — CA Scenario Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "Hyle Viewer",
        options,
        Box::new(|cc| {
            app::ViewerApp::new(cc, CpuSolverProvider::new())
                .map(|app| Box::new(app) as Box<dyn eframe::App>)
                .map_err(|error| Box::new(Error::other(error)) as _)
        }),
    )
}
