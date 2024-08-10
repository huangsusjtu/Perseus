use eframe::egui::Context;
use eframe::*;
use crate::app::WrapApp;

impl WrapApp {
    pub (crate) fn top_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.label("top Hello World!");

            show_menu(ui);
        });

        fn show_menu(ui: &mut egui::Ui) {
            use egui::{menu, Button};

            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
            });
        }
    }

    pub (crate) fn left_side_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("left Hello World!");
        });

    }

    pub (crate) fn right_side_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::SidePanel::right("my_right_panel").show(ctx, |ui| {
            ui.label("right Hello World!");
        });

    }

    pub (crate) fn bottom_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.label("bottom Hello World!");
        });
    }

    pub (crate) fn content(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

        });
    }


}