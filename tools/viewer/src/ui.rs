//! UI panel drawing functions.

use eframe::egui;
use hyle_core::voxel::MaterialId;
use hyle_core::MaterialAccess;

use crate::tools::{HoverInfo, Tool};
use crate::world::Materials;

/// Draw the top toolbar. Returns true if simulation step was requested.
pub fn draw_top_bar(
    ctx: &egui::Context,
    tool: &mut Tool,
    fps: f64,
    viewport_size: (u32, u32),
) -> bool {
    let mut step_sim = false;

    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for t in [Tool::Place, Tool::Delete, Tool::Inspect] {
                if ui.selectable_label(*tool == t, t.label()).clicked() {
                    *tool = t;
                }
            }

            ui.separator();

            if ui.button("Step Gravity").clicked() {
                step_sim = true;
            }

            ui.separator();

            if ui.button("Reset").clicked() {
                // handled externally via return
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!(
                    "{:.0} fps | {}x{}",
                    fps, viewport_size.0, viewport_size.1
                ));
            });
        });
    });

    step_sim
}

/// Draw the side panel with material selector and properties.
/// Returns the currently selected material ID.
pub fn draw_side_panel(
    ctx: &egui::Context,
    materials: &Materials,
    selected: &mut MaterialId,
) {
    egui::SidePanel::left("materials")
        .default_width(180.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Materials");
            ui.separator();

            // List materials (skip air at index 0)
            for id in 1..materials.count() {
                let mat_id = id as MaterialId;
                let def = materials.get_material(mat_id);

                let is_selected = *selected == mat_id;

                ui.horizontal(|ui| {
                    // Color swatch
                    let c = &def.visual.color;
                    let srgb = egui::Color32::from_rgb(
                        (c[0] * 255.0) as u8,
                        (c[1] * 255.0) as u8,
                        (c[2] * 255.0) as u8,
                    );
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(14.0, 14.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(rect, 2.0, srgb);

                    if ui.selectable_label(is_selected, &def.name).clicked() {
                        *selected = mat_id;
                    }
                });
            }

            ui.separator();
            ui.heading("Properties");
            ui.separator();

            // Show selected material properties
            let def = materials.get_material(*selected);

            ui.label(format!("Name: {}", def.name));

            let c = &def.visual.color;
            let srgb = egui::Color32::from_rgb(
                (c[0] * 255.0) as u8,
                (c[1] * 255.0) as u8,
                (c[2] * 255.0) as u8,
            );
            ui.horizontal(|ui| {
                ui.label("Color:");
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(20.0, 14.0),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(rect, 2.0, srgb);
            });

            ui.label(format!("Phase: {:?}", def.phase.state));
            ui.label(format!("Density: {:.0} kg/m³", def.structural.density));
            ui.label(format!("Roughness: {:.2}", def.visual.roughness));

            if def.visual.transmittance > 0.0 {
                ui.label(format!("Transmittance: {:.2}", def.visual.transmittance));
                ui.label(format!("IOR: {:.2}", def.visual.ior));
            }

            if let Some(mp) = def.thermal.melting_point {
                ui.label(format!("Melting: {:.0}°C", mp));
            }
            if let Some(fp) = def.thermal.freezing_point {
                ui.label(format!("Freezing: {:.0}°C", fp));
            }
        });
}

/// Draw the status bar at the bottom.
pub fn draw_status_bar(
    ctx: &egui::Context,
    tool: Tool,
    selected_name: &str,
    hover: &Option<HoverInfo>,
) {
    egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if let Some(info) = hover {
                ui.label(format!(
                    "Cursor: ({}, {}, {})  |  Voxel: {}",
                    info.world_pos.0, info.world_pos.1, info.world_pos.2,
                    info.material_name,
                ));
            } else {
                ui.label("Cursor: —");
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("Tool: {} {}", tool.label(), selected_name));
            });
        });
    });
}
