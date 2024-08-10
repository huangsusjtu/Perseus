use std::time::Duration;

use eframe::egui::{Context, RawInput, Visuals};
use eframe::*;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {}

pub struct WrapApp {
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) state: State,
    pub(crate) dropped_files: Vec<egui::DroppedFile>,
}

impl WrapApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            name: "".to_string(),
            age: 0,
            state: State::default(),
            dropped_files: Default::default(),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        slf
    }
}

impl eframe::App for WrapApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) =
            frame.info().web_info.location.hash.strip_prefix('#')
        {
            let anchor =
                Anchor::all().into_iter().find(|x| x.to_string() == anchor);
            if let Some(v) = anchor {
                self.state.selected_anchor = v;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ctx
            .input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11))
        {
            let fullscreen =
                ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                !fullscreen,
            ));
        }

        self.top_bar(ctx, frame);
        self.bottom_bar(ctx, frame);
        self.left_side_bar(ctx, frame);
        self.right_side_bar(ctx, frame);
        self.content(ctx, frame);
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn on_exit(&mut self, _gl: Option<&glow::Context>) {}

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        // Give the area behind the floating windows a different color, because
        // it looks better:
        let color = egui::lerp(
            egui::Rgba::from(visuals.panel_fill)
                ..=egui::Rgba::from(visuals.extreme_bg_color),
            0.5,
        );
        let color = egui::Color32::from(color);
        color.to_normalized_gamma_f32()
    }
}
