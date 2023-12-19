use egui;

pub struct JSONView {}

impl Default for JSONView {
    fn default() -> Self {
        Self {}
    }
}

impl JSONView {
    pub fn ui(ui: &mut egui::Ui) -> egui::Response {
        ui.heading("hello json view world")
    }
}
