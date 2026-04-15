//! Minimal UI — top toolbar with scene selection, sim controls, and FPS.

use eframe::egui;
use hyle_ca_analysis::{RuntimeReport, SpecAnalysis};
use hyle_ca_interface::MaterialSet;

use crate::ca::{Scenario, ViewerCell};

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
    show_runtime_analysis: &mut bool,
    show_static_analysis: &mut bool,
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
            ui.checkbox(show_runtime_analysis, "Runtime Analysis");
            ui.checkbox(show_static_analysis, "Static Analysis");

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

pub fn draw_static_analysis_window(
    ctx: &egui::Context,
    open: &mut bool,
    analysis: &SpecAnalysis,
) {
    egui::Window::new("Static Analysis")
        .open(open)
        .resizable(true)
        .default_width(360.0)
        .show(ctx, |ui| {
            ui.label(format!("Rules: {}", analysis.summary.rule_count));
            ui.label(format!(
                "Neighborhoods: {}",
                analysis.summary.neighborhood_count
            ));
            ui.label(format!("Attributes: {}", analysis.summary.attribute_count));
            ui.label(format!("Materials: {}", analysis.summary.materials.len()));
            ui.label(format!("Max radius: {}", analysis.summary.max_radius));
            ui.separator();

            ui.collapsing("Neighborhood usage", |ui| {
                for neighborhood in &analysis.neighborhoods {
                    ui.label(format!(
                        "{} | radius {} | used by {} rule(s)",
                        neighborhood.name,
                        neighborhood.spec.radius().get(),
                        neighborhood.used_by_rules
                    ));
                }
            });

            ui.collapsing("Diagnostics", |ui| {
                if analysis.all_diagnostics().next().is_none() {
                    ui.label("No diagnostics.");
                } else {
                    for diagnostic in analysis.all_diagnostics() {
                        ui.label(format!(
                            "[{}] {}: {}",
                            diagnostic.severity, diagnostic.code, diagnostic.message
                        ));
                    }
                }
            });
        });
}

pub fn draw_runtime_analysis_window(
    ctx: &egui::Context,
    open: &mut bool,
    analysis: Option<&RuntimeReport>,
) {
    egui::Window::new("Runtime Analysis")
        .open(open)
        .resizable(true)
        .default_width(360.0)
        .show(ctx, |ui| match analysis {
            Some(analysis) => {
                ui.label(format!("Step: {}", analysis.step));
                ui.label(format!("Living cells: {}", analysis.living_cells));
                ui.label(format!("Changed cells: {}", analysis.changed_cells));
                ui.label(format!("Stable cells: {}", analysis.stable_cells));
                ui.label(format!("Born this step: {}", analysis.born_cells));
                ui.label(format!("Died this step: {}", analysis.died_cells));
                ui.separator();

                ui.collapsing("Populations", |ui| {
                    for population in &analysis.populations {
                        if let Some(material) = ViewerCell::from_material_id(population.material) {
                            ui.label(format!("{}: {}", material.label(), population.count));
                        } else {
                            ui.label(format!("material {}: {}", population.material.raw(), population.count));
                        }
                    }
                });

                ui.collapsing("Transitions", |ui| {
                    if analysis.transitions.is_empty() {
                        ui.label("No material transitions.");
                    } else {
                        for transition in &analysis.transitions {
                            let from = ViewerCell::from_material_id(transition.from)
                                .map(|material| material.label().to_owned())
                                .unwrap_or_else(|| format!("material {}", transition.from.raw()));
                            let to = ViewerCell::from_material_id(transition.to)
                                .map(|material| material.label().to_owned())
                                .unwrap_or_else(|| format!("material {}", transition.to.raw()));
                            ui.label(format!("{from} -> {to}: {}", transition.count));
                        }
                    }
                });
            }
            None => {
                ui.label("Runtime analysis appears after the next completed step.");
            }
        });
}
