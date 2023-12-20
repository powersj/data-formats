use egui;

pub struct LineProtocolView {
    code: String,
}

impl Default for LineProtocolView {
    fn default() -> Self {
        Self {
            code: String::new().into(),
        }
    }
}

impl LineProtocolView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { code } = self;
        ui.heading("Enter line protocol below:");
        ui.add(egui::TextEdit::multiline(code).code_editor());
    }
}
