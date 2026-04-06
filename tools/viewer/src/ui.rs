//! Minimal UI — just a top toolbar with auto-step controls and FPS.

use eframe::egui;

/// Draw the top toolbar. Returns (step_requested, reset_requested).
pub fn draw_toolbar(
    ctx: &egui::Context,
    auto_step: &mut bool,
    step_interval_ms: &mut f64,
    fps: f64,
    viewport_size: (u32, u32),
) -> (bool, bool) {
    let mut step = false;
    let mut reset = false;

    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(auto_step, "Auto");

            if *auto_step {
                let mut interval_f = *step_interval_ms as f32;
                ui.add(
                    egui::Slider::new(&mut interval_f, 16.0..=1000.0)
                        .text("ms")
                        .logarithmic(true),
                );
                *step_interval_ms = interval_f as f64;
            }

            ui.separator();

            if ui.button("Step").clicked() {
                step = true;
            }
            if ui.button("Reset").clicked() {
                reset = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!(
                    "{:.0} fps  |  {}×{}",
                    fps, viewport_size.0, viewport_size.1
                ));
            });
        });
    });

    (step, reset)
}
