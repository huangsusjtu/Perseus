#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(clippy::never_loop)] // False positive

use crate::app::PerseusApp;
use iced::{Sandbox, Settings};
use libutil::log;

mod app;

mod scene;

// #[global_allocator]
// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn run() -> anyhow::Result<()> {
    let _log_cleaner = log::init_log()?;
    PerseusApp::run(Settings::default()).expect("TODO: panic message");
    Ok(())
}

// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}
