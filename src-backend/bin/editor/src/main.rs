#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(clippy::never_loop)] // False positive

mod app;
mod menu;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use eframe::*;

fn main() -> Result<(), eframe::Error> {
    let _logger = libutil::log::init_log().unwrap();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1080.0, 900.0])
            .with_resizable(true)
            .with_drag_and_drop(true),
        centered: true,
        default_theme: Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "perseus-editor",
        options,
        Box::new(|cc| Box::new(app::WrapApp::new(cc))),
    )
}
