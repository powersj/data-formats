use eframe::egui;
use std::process;

mod json;
mod line_protocol;

pub const APP_NAME: &str = "formats";
pub const TXT_WELCOME: &str = "Welcome! Click on a format to the left to get started";
pub const VIEW_LP: &str   = "line protocol";
pub const VIEW_JSON: &str = "json         ";

#[derive(PartialEq, Eq)]
enum Panel {
    Default,
    Json,
    LineProtocol,
}

impl Default for Panel {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Default)]
struct MyApp {
    open_panel: Panel,
    show_about: bool,
    json_view: json::JSONView,
    line_protocol_view: line_protocol::LineProtocolView,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        if self.show_about == true {
            egui::Window::new("About")
                .default_width(320.0)
                .default_height(480.0)
                .open(&mut self.show_about)
                .show(ctx, |ui| {
                    ui.label("formats is a tool for formatting and validating data formats.");
                    ui.label("It is written in Rust and uses the egui framework.");
                    ui.label("It is open source and available on GitHub:");
                    ui.hyperlink_to("\t* GitHub", "https://github.com/powersj/formats");
                    ui.hyperlink_to("\t* egui GitHub", "https://github.com/emilk/egui");
                    ui.hyperlink_to("\t* egui Demo", "https://www.egui.rs/#demo");
                    ui.label("");
                });
        }
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(APP_NAME);
                ui.menu_button("   file  ", |ui| {
                    if ui.button("exit").clicked() {
                       process::exit(0);
                    }
                });
                if ui.button("  about  ").clicked() {
                    self.show_about = true;
                }
            });
        });
        egui::SidePanel::left("side").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.open_panel, Panel::Json, VIEW_JSON);
                ui.selectable_value(&mut self.open_panel, Panel::LineProtocol, VIEW_LP);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.open_panel {
                Panel::Default => {
                    ui.centered_and_justified(|ui| {
                        ui.heading(TXT_WELCOME)
                });
                }
                Panel::Json => {
                    json::JSONView::ui(&mut self.json_view, ui);
                }
                Panel::LineProtocol => {
                    line_protocol::LineProtocolView::ui(&mut self.line_protocol_view, ui);
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 1080.0]),
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
