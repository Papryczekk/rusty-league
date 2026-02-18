#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

mod app;
mod launcher;
mod credentials;
mod settings;

use app::RustyLeagueApp;
use eframe::egui;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let icon_bytes = include_bytes!("../assets/icon.ico");
    let icon_image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = icon_image.dimensions();
    let icon_data = egui::IconData {
        rgba: icon_image.into_raw(),
        width,
        height,
    };

    let simple_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([300.0, 220.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_icon(Arc::new(icon_data)),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty League",
        simple_options,
        Box::new(|cc| Ok(Box::new(RustyLeagueApp::new(cc)))),
    )
}
