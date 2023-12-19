use egui;

pub struct LineProtocolView {}

impl Default for LineProtocolView {
    fn default() -> Self {
        Self {}
    }
}

impl LineProtocolView {
    pub fn ui(ui: &mut egui::Ui) -> egui::Response {
        ui.heading("line protocol view")
    }
}
