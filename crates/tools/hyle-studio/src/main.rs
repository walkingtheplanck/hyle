use eframe::egui;

struct HyleStudioApp;

impl eframe::App for HyleStudioApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let _ = ui.button("compile");
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Hyle Studio",
        native_options,
        Box::new(|_cc| Ok(Box::new(HyleStudioApp))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn start(canvas: web_sys::HtmlCanvasElement) -> Result<(), wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();

    eframe::WebRunner::new()
        .start(canvas, web_options, Box::new(|_cc| Ok(Box::new(HyleStudioApp))))
        .await
}
