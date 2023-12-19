use std::process;

use egui;

pub fn load(app_name: &str, ui: &mut egui::Ui) {
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

        }
    });
}
