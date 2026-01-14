#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aip_app::app::App;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Summarizer",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
