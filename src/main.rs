use eframe::egui;

mod json;
mod line_protocol;
mod toolbar;

pub const APP_NAME: &str = "formats";
pub const TXT_WELCOME: &str = "Welcome!";
pub const VIEW_LP: &str = "line protocol";
pub const VIEW_JSON: &str = "json";

#[derive(PartialEq, Eq)]
enum Panel {
    Default,
    JSON,
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
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            toolbar::load(APP_NAME, ui);
        });
        egui::SidePanel::left("side").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.open_panel, Panel::JSON, VIEW_JSON);
                ui.selectable_value(&mut self.open_panel, Panel::LineProtocol, VIEW_LP);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.open_panel {
                Panel::Default => { ui.heading(TXT_WELCOME); }
                Panel::JSON => { json::JSONView::ui(ui); }
                Panel::LineProtocol => { line_protocol::LineProtocolView::ui(ui); }
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
