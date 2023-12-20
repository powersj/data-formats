use egui;

pub struct JSONView {
    input: String,
}

impl Default for JSONView {
    fn default() -> Self {
        Self {
            input: String::new().into(),
        }
    }
}

impl JSONView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { input } = self;
        ui.heading("Enter JSON below:");
        ui.add(egui::TextEdit::multiline(input).code_editor());
    }
}
