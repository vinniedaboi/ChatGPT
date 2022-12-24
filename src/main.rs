use eframe::egui;
use eframe::egui::{Color32, FontFamily, FontId, TextStyle};
use std::collections::HashMap;

//const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
//const BLUE: Color32 = Color32::from_rgb(0,0,255);
pub const PADDING: f32 = 5.0;
struct App {
    prompt: String,
    response: String,
}
impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        Self {
            prompt: "Enter Chat Prompt".to_owned(),
            response: "Rust is pretty pog".to_owned(),
        }
    }
}
fn buttonclicked() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_source("Scroll")
                .show(ui, |ui| {
                    ui.heading("Chat Prompt");
                    ui.separator();
                    ui.add_sized([785., 200.], egui::TextEdit::multiline(&mut self.prompt));
                    ui.separator();
                    let process = ui.add_sized([785., 40.], egui::Button::new("-->"));
                    if process.clicked() {
                        buttonclicked().ok();
                    }
                    ui.separator();
                    ui.heading("OpenAI:");
                    ui.add_space(10.);
                    ui.colored_label(Color32::from_rgb(0, 0, 255), &mut self.response);
                    ui.separator();
                    let new_chat = ui.add_sized([785., 40.], egui::Button::new("New Thread"));
                    if new_chat.clicked() {
                        buttonclicked().ok();
                    }
                });
        });
    }
}
fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(26.0, Proportional)),
        (TextStyle::Body, FontId::new(15.0, Proportional)),
        (TextStyle::Monospace, FontId::new(15.0, Monospace)),
        (TextStyle::Button, FontId::new(15.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "/Users/vincentngsoonzheng/vinnie-dev/Rust/learning/MesloLGS_NF_Regular.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 440.0)),
        ..Default::default()
    };
    eframe::run_native("OpenAI", options, Box::new(|cc| Box::new(App::new(cc))))
}
