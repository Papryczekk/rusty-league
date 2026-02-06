#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

mod app;
mod launcher;
mod credentials;
mod settings;

use app::RustyLeagueApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let simple_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([300.0, 220.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty League",
        simple_options,
        Box::new(|cc| Ok(Box::new(RustyLeagueApp::new(cc)))),
    )
}
