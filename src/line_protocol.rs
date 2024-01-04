#[derive(Default)]
pub struct LineProtocolView {
    input: String,
    error: String,
}

impl LineProtocolView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { input, error } = self;

        ui.horizontal(|ui| {
            if ui.button("analyze").clicked() {
                *error = parse_line_protocol(input);
            }
            ui.heading("<-- Press to validate and format line protocol");
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


fn parse_line_protocol(lines: &str) -> String {
    let mut errors: Vec<String> = vec![];
    let _lines: Vec<_> = influxdb_line_protocol::parse_lines(lines)
        .filter_map(|r| r.map_err(|e| errors.push(e.to_string())).ok())
        .collect();

    if errors.is_empty() {
        "valid line protocol!".to_string()
    } else {
        errors[0].to_string()
    }
}
