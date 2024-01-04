#[derive(Default)]

pub struct JSONView {
    input: String,
    error: String,
}

impl JSONView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { input, error } = self;
        ui.horizontal(|ui| {
            if ui.button("analyze").clicked() {
                let result = parse_json(input);
                match result {
                    Ok(_) => {
                        *input = pretty_json(input);
                        *error = "valid JSON!".to_string();
                    },
                    Err(e) => *error = e.to_string(),
                 }
            }
            ui.heading("<-- Press to validate and format JSON");
        });
        ui.label(error.to_string());
        ui.add(
            egui::TextEdit::multiline(input)
            .code_editor()
            .desired_width(ui.available_width())
            .desired_rows(40)
            .font(eframe::egui::TextStyle::Monospace)
        );
    }
}

fn parse_json(input: &str) -> Result<serde_json::Value, serde_json::Error>{
    serde_json::from_str::<serde_json::Value>(input)
}

fn pretty_json(input: &str) -> String {
    jsonxf::pretty_print(input).unwrap()
}
