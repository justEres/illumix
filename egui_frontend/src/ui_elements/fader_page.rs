use eframe::CreationContext;
use eframe::egui::{
    self, Button, CentralPanel, ColorImage, Context, Slider, TextureHandle, Ui, UiBuilder, Vec2, Window
};
use eframe::egui::{Color32, Pos2, Rect, Style};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::fader_page::fader::Fader;

mod fader;

pub struct FaderPage {
    fader_list: [fader::Fader; 24],

    //Styling
    panel_resolution: Vec2,
}

impl FaderPage {
    pub fn new(ctx: &CreationContext) -> Self {
        let panel_resolution = Vec2 { x: 1000., y: 800. };

        let fader_list: [fader::Fader; 24] = std::array::from_fn(|_| fader::Fader::new(None, None));

        Self {
            fader_list,
            panel_resolution,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.panel_resolution = Vec2 { x: ui.min_size().x, y: ui.min_size().y };
    
            // Styling (unchanged except your values)
            let mut style = (*ctx.style()).clone();
            style.spacing.slider_width = ((self.panel_resolution.y / 100.) * 37.5);
            style.spacing.interact_size.y = ((self.panel_resolution.y / 100.) * 6.);
            style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };
            ui.set_style(style.clone());
    
            let row_gap = 40.0;
    
            // helper to draw one fader column (unchanged)
            let mut draw_fader = |ui: &mut egui::Ui, i: usize| {
                ui.vertical(|ui| {
                    let slider = Slider::new(&mut self.fader_list[i].fader_value, 0..=255)
                        .vertical()
                        .show_value(false);
                    let slider_response = ui.add(slider);
    
                    ui.add_space(5.0);
    
                    let button_width  = ((self.panel_resolution.y / 100.) * 6.);
                    let button_height = ui.spacing().interact_size.y;
                    let slider_width  = slider_response.rect.width();
                    let offset        = (slider_width - button_width) / 2.0;
    
                    let button_rect = Rect::from_min_size(
                        Pos2 {
                            x: slider_response.rect.left() + offset,
                            y: slider_response.rect.bottom() + 5.0,
                        },
                        Vec2 { x: button_width, y: button_height },
                    );
    
                    ui.allocate_new_ui(UiBuilder::new().max_rect(button_rect), |ui| {
                        if ui.add_sized([button_width, button_height], Button::new(format!("{}", i + 1))).clicked() {
                            alert(&format!("Ayo from slider {i}"));
                        }
                    });
                });
            };
    
            // --- function to draw a row that spreads columns evenly across available width
            let mut draw_row_even = |ui: &mut egui::Ui, start: usize, end: usize| {
                let n = end - start;
                if n == 0 { return; }
    
                // Each column is at least as wide as the slider (or the button if wider)
                let col_w = ui.style().spacing.slider_width
                    .max((self.panel_resolution.y / 100.) * 6.0);
    
                // Compute gap so columns + gaps exactly fill the row width (space-between)
                let avail = ui.available_width();
                let total_cols = col_w * n as f32;
                let gap = ((avail - total_cols) / (n as f32 - 1.0)).max(0.0);
    
                // Apply spacing only inside this scope
                ui.scope(|ui| {
                    // set spacing BEFORE adding the widgets in this row
                    ui.spacing_mut().item_spacing.x = gap; // gap you computed
                    ui.horizontal(|ui| {
                        for i in 0..12 {
                            draw_fader(ui, i);
                        }
                    });
                });
            };
    
            // Row 1: 0..12 and Row 2: 12..24, evenly spread
            draw_row_even(ui, 0, 12);
            ui.add_space(row_gap);
            draw_row_even(ui, 12, 24);
        });
    }
    
}


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}