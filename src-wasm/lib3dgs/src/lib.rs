#![allow(unused_assignments, unused_variables, dead_code)]

use wasm_bindgen::prelude::*;

mod renderer;
mod scene;
mod utils;

// #[wasm_bindgen(start)]
// pub fn dummy_main() {
// }

// #[wasm_bindgen]
// pub async fn run() {
//     utils::set_panic_hook();
//     renderer::main().await;
// }

#[wasm_bindgen]
pub async fn run_3dgs(canvas_id: &str) {
    utils::set_panic_hook();
    renderer::main(canvas_id).await;
}
