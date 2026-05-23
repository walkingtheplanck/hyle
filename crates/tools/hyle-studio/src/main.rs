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

            let editor = egui::TextEdit::multiline(&mut self.source)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_width(f32::INFINITY)
                .desired_rows(32);

            ui.add_sized(ui.available_size(), editor);
        });
    }

    fn renderer_panel(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Renderer");
                ui.label(format!("compile passes: {}", self.compile_count));
            });

            ui.separator();

            let available = ui.available_size();
            let (rect, _response) = ui.allocate_exact_size(available, egui::Sense::hover());
            let painter = ui.painter_at(rect);

            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(12, 14, 18));

            let margin = 28.0;
            let render_rect = rect.shrink(margin);
            let cell_count = 24;
            let cell_size = (render_rect.width().min(render_rect.height()) / cell_count as f32)
                .floor()
                .max(8.0);
            let grid_size = cell_size * cell_count as f32;
            let grid_origin = egui::pos2(
                render_rect.center().x - grid_size / 2.0,
                render_rect.center().y - grid_size / 2.0,
            );

            let grid_rect = egui::Rect::from_min_size(grid_origin, egui::vec2(grid_size, grid_size));
            painter.rect_filled(grid_rect, 4.0, egui::Color32::from_rgb(18, 22, 28));

            for y in 0..cell_count {
                for x in 0..cell_count {
                    let min = grid_origin + egui::vec2(x as f32 * cell_size, y as f32 * cell_size);
                    let max = min + egui::vec2(cell_size - 1.0, cell_size - 1.0);
                    let rect = egui::Rect::from_min_max(min, max);

                    let distance = ((x as i32 - 12).abs() + (y as i32 - 12).abs()) as u8;
                    let color = match (x + y + self.compile_count as usize) % 7 {
                        0 => egui::Color32::from_rgb(235, 90, 52),
                        1 if distance < 10 => egui::Color32::from_rgb(248, 180, 72),
                        2 | 3 => egui::Color32::from_rgb(74, 161, 99),
                        _ => egui::Color32::from_rgb(42, 58, 72),
                    };

                    painter.rect_filled(rect, 1.0, color);
                }
            }

            painter.rect_stroke(
                grid_rect,
                4.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 104, 122)),
                egui::StrokeKind::Inside,
            );
        });
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

        egui::CentralPanel::default().show_inside(ui, |ui| {
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
