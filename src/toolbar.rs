use std::process;

pub fn load(ctx: &egui::Context, app_name: &str, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.heading(app_name);
        ui.menu_button("file", |ui| {
            if ui.button("exit").clicked() {
                process::exit(0);
            }
        });
        if ui.button("settings").clicked() {

        }
        if ui.button("help").clicked() {

        }
        if ui.button("about").clicked() {
            egui::Window::new("About")
                .default_width(320.0)
                .default_height(480.0)
                .open(&mut true)
                .show(ctx, |ui| {
                    ui.label("formats is a tool for formatting and validating data formats.");
                    ui.label("It is written in Rust and uses the egui framework.");
                    ui.label("It is open source and available on GitHub.");
                    ui.label("");
                });
        }
    });
}
