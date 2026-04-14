//! Minimal UI — top toolbar with scene selection, sim controls, and FPS.

use eframe::egui;

use crate::ca::Scenario;

pub struct ToolbarActions {
    pub step_requested: bool,
    pub reset_requested: bool,
    pub scenario_selected: Option<Scenario>,
}

pub fn draw_toolbar(
    ctx: &egui::Context,
    current_scenario: Scenario,
    auto_step: &mut bool,
    step_interval_ms: &mut f64,
    fps: f64,
    viewport_size: (u32, u32),
) -> ToolbarActions {
    let mut actions = ToolbarActions {
        step_requested: false,
        reset_requested: false,
        scenario_selected: None,
    };

    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| {
            for scenario in Scenario::ALL {
                let selected = scenario == current_scenario;
                if ui.selectable_label(selected, scenario.label()).clicked() && !selected {
                    actions.scenario_selected = Some(scenario);
                }
            }

            ui.separator();
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
                actions.step_requested = true;
            }
            if ui.button("Reset").clicked() {
                actions.reset_requested = true;
            }

            ui.separator();
            ui.label(current_scenario.description());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!(
                    "{:.0} fps  |  {}×{}",
                    fps, viewport_size.0, viewport_size.1
                ));
            });
        });
    });

    actions
}
