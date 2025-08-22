use std::fmt::format;

use eframe::egui::{
    self, Button, CentralPanel, Color32, ColorImage, Context, Slider, TextureHandle, Ui, UiBuilder,
    Vec2, Window,
};
use eframe::egui::{Pos2, Rect};
use eframe::{CreationContext, Frame};
use fixture_lib::fixture::Dimmer;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::fader_page::fader::Fader;
use crate::fixture_component_listener::{ChangeEventManager, ListenerDatabase, SharedState};

mod fader;

pub struct FaderPage {
    fader_list: [fader::Fader; 24],

    //Styling
    panel_resolution: Vec2,
    change_event_manager: SharedState<ChangeEventManager>,
    listener_database: SharedState<ListenerDatabase>,
    rect: Rect,
}

impl FaderPage {
    pub fn new(
        ctx: &CreationContext,
        change_event_manager: SharedState<ChangeEventManager>,
        listener_database: SharedState<ListenerDatabase>,
    ) -> Self {
        let panel_resolution = Vec2 { x: 1000., y: 800. };

        let fader_list: [fader::Fader; 24] = std::array::from_fn(|_| fader::Fader::new(None, None));

        let rect = Rect::from_min_size(Pos2::ZERO, egui::vec2(0.0, 0.0));

        Self {
            fader_list,
            panel_resolution,
            rect,
            change_event_manager,
            listener_database,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.rect = ui.max_rect();
                self.panel_resolution = Vec2 {
                    x: self.rect.max.x / 100.,
                    y: self.rect.max.y / 100.,
                };

                let mut style = (*ctx.style()).clone();
                style.spacing.slider_width = ((self.panel_resolution.y) * 37.5); // Wider slider
                style.spacing.interact_size.y = ((self.panel_resolution.y) * 6.); // Taller handle
                style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };

                ui.set_style(style.clone());

                self.draw_slider_bank(ui);
            });
        });
    }

    fn draw_slider_bank(&mut self, ui: &mut Ui) {
        for i in 0..24 {
            let mut offset = Vec2 { x: 0., y: 0. };
            if (i > 5 && i < 12) || (i > 17) {
                offset = Vec2 {
                    x: (self.panel_resolution.x * 6.),
                    y: 0.,
                };
            }

            if i < 12 {
                offset += Vec2 {
                    x: (self.panel_resolution.x * 6.) * i as f32,
                    y: 0.,
                };
            } else {
                offset += Vec2 {
                    x: (self.panel_resolution.x * 6.) * (i as f32 - 12.),
                    y: self.panel_resolution.y * 50.,
                };
            }

            let local = egui::Rect::from_min_size(self.rect.min + offset, egui::vec2(20.0, 20.0));

            let response = ui.put(
                local,
                egui::Slider::new(&mut self.fader_list[i].fader_value, 0..=255)
                    .show_value(false)
                    .orientation(egui::SliderOrientation::Vertical),
            );

            if response.changed() {
                self.fader_list[i].fader_value_changed();
                self.change_event_manager.borrow_mut().create_event(
                    i as u8,
                    0,
                    fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer {
                        intensity: self.fader_list[i].fader_value,
                    }),
                );
            }

            let local = egui::Rect::from_min_size(
                self.rect.min
                    + offset
                    + Vec2 {
                        x: 0.,
                        y: self.panel_resolution.y * 40.,
                    },
                egui::vec2(self.panel_resolution.y * 6., 20.0),
            );

            if self.fader_list[i].is_selected == false
                && ui.put(local, Button::new(format!("{}", i + 1))).clicked()
            {
                self.fader_list[i].is_selected = true;
            } else if self.fader_list[i].is_selected == true
                && ui.put(local, Button::new(format!("{}", i + 1))).clicked()
            {
                self.fader_list[i].is_selected = false;
            }

            let local = egui::Rect::from_min_size(
                self.rect.min
                    + offset
                    + Vec2 {
                        x: self.panel_resolution.y * 2.5,
                        y: self.panel_resolution.y * 40.5,
                    },
                egui::vec2(self.panel_resolution.y * 1., 9.),
            );

            ui.allocate_rect(local, egui::Sense::hover());

            let mut color = Color32::from_rgb(0, 0, 0);

            if self.fader_list[i].is_selected == true {
                color = Color32::from_rgb(255, 200, 120);
            }

            ui.painter().rect_filled(
                local, 5.0,   // corner radius
                color, // background color
            );
        }
    }
}

