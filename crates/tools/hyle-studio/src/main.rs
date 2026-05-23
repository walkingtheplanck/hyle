use eframe::egui;

struct HyleStudioApp {
    source: String,
    compile_count: u32,
}

impl HyleStudioApp {
    fn new() -> Self {
        Self {
            source: include_str!("../../../../examples/game.hyle").to_owned(),
            compile_count: 0,
        }
    }

    fn editor_panel(&mut self, ui: &mut egui::Ui) {
        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Hyle Script");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("compile").clicked() {
                        self.compile_count = self.compile_count.saturating_add(1);
                    }
                });
            });

            ui.separator();

            let editor_size = ui.available_size();
            let editor_rows = self.source.lines().count().max(32);

            ui.allocate_ui_with_layout(
                editor_size,
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::ScrollArea::both()
                        .id_salt("script_editor_scroll")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let editor = egui::TextEdit::multiline(&mut self.source)
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .desired_width(ui.available_width().max(720.0))
                                .desired_rows(editor_rows);

                            ui.add(editor);
                        });
                },
            );
        });
    }

    fn renderer_panel(&self, ui: &mut egui::Ui) {
        let (canvas_rect, _response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter_at(canvas_rect).with_clip_rect(canvas_rect);

        painter.rect_filled(canvas_rect, 0.0, egui::Color32::from_rgb(9, 11, 15));

        let camera_rect = canvas_rect.shrink(24.0);
        let world_cells = 24;
        let world_size = egui::vec2(world_cells as f32, world_cells as f32);
        let zoom = (camera_rect.width() / world_size.x).min(camera_rect.height() / world_size.y);
        let grid_size = world_size * zoom;
        let grid_min = camera_rect.center() - grid_size / 2.0;
        let grid_rect = egui::Rect::from_min_size(grid_min, grid_size);

        painter.rect_filled(grid_rect, 4.0, egui::Color32::from_rgb(18, 22, 28));

        for y in 0..world_cells {
            for x in 0..world_cells {
                let cell_min = grid_min + egui::vec2(x as f32 * zoom, y as f32 * zoom);
                let cell_max = cell_min + egui::vec2(zoom, zoom);
                let cell_rect = egui::Rect::from_min_max(cell_min, cell_max).shrink(0.5);

                let distance = ((x as i32 - 12).abs() + (y as i32 - 12).abs()) as u8;
                let color = match (x + y + self.compile_count as usize) % 7 {
                    0 => egui::Color32::from_rgb(235, 90, 52),
                    1 if distance < 10 => egui::Color32::from_rgb(248, 180, 72),
                    2 | 3 => egui::Color32::from_rgb(74, 161, 99),
                    _ => egui::Color32::from_rgb(42, 58, 72),
                };

                painter.rect_filled(cell_rect, 1.0, color);
            }
        }

        painter.rect_stroke(
            grid_rect,
            4.0,
            egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 104, 122)),
            egui::StrokeKind::Inside,
        );

        let label_pos = canvas_rect.left_top() + egui::vec2(16.0, 14.0);
        painter.text(
            label_pos,
            egui::Align2::LEFT_TOP,
            format!("Renderer  compile passes: {}", self.compile_count),
            egui::FontId::proportional(14.0),
            egui::Color32::from_rgb(190, 196, 205),
        );
    }
}

impl eframe::App for HyleStudioApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::right("script_editor")
            .resizable(true)
            .default_size(440.0)
            .size_range(320.0..=720.0)
            .show_inside(ui, |ui| {
                self.editor_panel(ui);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show_inside(ui, |ui| {
                self.renderer_panel(ui);
            });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Hyle Studio",
        native_options,
        Box::new(|_cc| Ok(Box::new(HyleStudioApp::new()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn start(canvas: web_sys::HtmlCanvasElement) -> Result<(), wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();

    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(|_cc| Ok(Box::new(HyleStudioApp::new()))),
        )
        .await
}
