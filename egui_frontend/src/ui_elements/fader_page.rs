use eframe::CreationContext;
use eframe::egui::{
    self, Button, CentralPanel, ColorImage, Context, Slider, TextureHandle, Ui, UiBuilder, Vec2, Window
};
use eframe::egui::{Color32, Pos2, Rect, Style};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::fader_page::fader::Fader;

mod fader;

pub struct FaderPage {
    //fader_list: [fader::Fader; 24]
    test_fader: fader::Fader,

    //Styling
    panel_resolution: Vec2,
}

impl FaderPage {
    pub fn new(ctx: &CreationContext) -> Self {
        let panel_resolution = Vec2 { x: 1000., y: 800. };

        Self {
            test_fader: Fader::new(0., 0., None, None),
            panel_resolution,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("test");
            //self.panel_resolution = Vec2 { x: ui.min_size().x, y: ui.min_size().y };

            /// Styling
            let mut style = (*ctx.style()).clone();
            style.spacing.slider_width = ((self.panel_resolution.y / 100.) * 37.5); // Wider slider
            style.spacing.interact_size.y = ((self.panel_resolution.y / 100.) * 6.); // Taller handle
            style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };

            ui.set_style(style.clone());

            let widget_rect = Rect::from_min_size(
                Pos2 {
                    x: self.test_fader.pos_x,
                    y: self.test_fader.pos_y,
                },
                Vec2 { x: 100., y: 100. },
            );


            let mut values = vec![0, 0, 0, 0, 0];
            ui.horizontal(|ui| {
        for i in 0..5 {
            ui.vertical(|ui| {
                // Vertical slider
                let slider = Slider::new(&mut values[i], 0..=100)
                    .vertical()
                    .show_value(false);
                let slider_response = ui.add(slider);

                // Space between slider and button
                ui.add_space(5.0);

                // Center the button under the slider
                let button_width = 30.0; // approximate width of your button
                let slider_width = slider_response.rect.width();
                let offset = (slider_width - button_width) / 2.0;

                ui.allocate_new_ui(
                    UiBuilder::new().max_rect(slider_response.rect.translate(egui::vec2(offset, slider_response.rect.height() + 5.0))),
                    
                    |ui| {
                        if ui.button(" Käse ").clicked() {
                            alert(&format!("Ayo from slider {i}"));
                        }
                    },
                );
            });

            // Space between channels
            ui.add_space(10.0);
        }
    });
        });
    }
}


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}