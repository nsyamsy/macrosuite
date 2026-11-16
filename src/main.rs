#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod theme;
mod fs_engine;
mod excel_engine;
mod app;

use app::MacroSuiteApp;
use eframe::egui::{IconData, Vec2, ViewportBuilder};
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let icon = Arc::new(IconData {
        rgba: include_bytes!("../assets/icon_64.rgba").to_vec(),
        width: 64,
        height: 64,
    });

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("MacroSuite Studio")
            .with_inner_size(Vec2::new(1160.0, 780.0))
            .with_min_inner_size(Vec2::new(820.0, 540.0))
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "MacroSuite Studio",
        native_options,
        Box::new(|cc| {
            theme::setup_custom_fonts(&cc.egui_ctx);
            theme::apply_custom_theme(&cc.egui_ctx, true);
            Ok(Box::new(MacroSuiteApp::default()))
        }),
    )
}
