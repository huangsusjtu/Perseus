#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(clippy::never_loop)] // False positive

mod app;
mod menu;

use eframe::*;
// use three_d::*;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn run() -> Result<(), eframe::Error> {
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

// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

// pub async fn run() {
//     let window = Window::new(WindowSettings {
//         title: "perseus-editor".to_string(),
//         max_size: Some((1280, 720)),
//         ..Default::default()
//     })
//         .unwrap();
//     let context = window.gl();
//
//     let mut camera = Camera::new_perspective(
//         window.viewport(),
//         vec3(3.0, 2.5, 6.0),
//         vec3(0.0, 1.5, 0.0),
//         vec3(0.0, 1.0, 0.0),
//         degrees(45.0),
//         0.1,
//         1000.0,
//     );
//     let mut control = OrbitControl::new(*camera.target(), 1.0, 1000.0);
//
//     // Source: https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0
//     let mut loaded = if let Ok(loaded) =
//         three_d_asset::io::load_async(&["/home/huangsu/work/github/three-d/
// examples/assets/obj/EQ5.obj"]).await     {
//         loaded
//     } else {
//         return;
//     };
//
//     let mut cpu_model: CpuModel = loaded.deserialize("EQ5").unwrap();
//     cpu_model
//         .geometries
//         .iter_mut()
//         .for_each(|part| part.compute_normals());
//     let mut model = Model::<PhysicalMaterial>::new(&context,
// &cpu_model).unwrap();
//
//     let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE,
// &vec3(0.0, -0.5, -0.5));     let light1 = DirectionalLight::new(&context,
// 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));
//
//     // main loop
//     window.render_loop(move |mut frame_input| {
//         camera.set_viewport(frame_input.viewport);
//         control.handle_events(&mut camera, &mut frame_input.events);
//
//         model.animate(0.001 * frame_input.accumulated_time as f32);
//
//         frame_input
//             .screen()
//             .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
//             .render(&camera, &model, &[&light0, &light1]);
//
//         FrameOutput::default()
//     });
// }
