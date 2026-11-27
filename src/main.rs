#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod icon;
mod monitor;
mod theme;
mod widgets;

use app::{CARD_SIZE, COMPACT_SIZE, NetworkMonitorApp};
use config::{AppConfig, ViewMode};
use eframe::egui::{self, ViewportBuilder, WindowLevel};
use std::sync::Arc;

fn main() -> eframe::Result {
    let config = AppConfig::load();
    let initial_size = match config.view_mode {
        ViewMode::Card => CARD_SIZE,
        ViewMode::Compact => COMPACT_SIZE,
    };

    let mut viewport = ViewportBuilder::default()
        .with_title("Network Monitor")
        .with_inner_size(initial_size)
        .with_min_inner_size(COMPACT_SIZE)
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(false)
        .with_icon(Arc::new(icon::generate_window_icon()))
        .with_window_level(if config.always_on_top {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        });

    if let Some([x, y]) = config.window_pos
        && x.is_finite()
        && y.is_finite()
        && x > -4000.0
        && x < 8000.0
        && y > -2000.0
        && y < 5000.0
    {
        viewport = viewport.with_position(egui::pos2(x, y));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Network Monitor",
        options,
        Box::new(move |_cc| Ok(Box::new(NetworkMonitorApp::new(config)))),
    )
}
