use std::fmt::format;


use eframe::{CreationContext, Frame};
use eframe::egui::{
    self, Button, CentralPanel, ColorImage, Context, Slider, TextureHandle, Ui, UiBuilder, Vec2, Window
};
use eframe::egui::{Color32, Pos2, Rect, Style};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Intl::DateTimeFormat;

use crate::fader_page::fader::Fader;
use crate::fader_page::ui_auto_scalling::AutoScaller;

#[path = "ui_helper/ui_auto_scalling.rs"]
mod ui_auto_scalling;

mod fader;

pub struct FaderPage {
    fader_list: [fader::Fader; 24],

    //Styling
    panel_resolution: Vec2,

    ui_auto_scaller: ui_auto_scalling::AutoScaller,

    rect: Rect,
}

impl FaderPage {
    pub fn new(ctx: &CreationContext) -> Self {
        let panel_resolution = Vec2 { x: 1000., y: 800. };
        
        let fader_list: [fader::Fader; 24] = std::array::from_fn(|_| fader::Fader::new(None, None));

        let rect = Rect::from_min_size(Pos2::ZERO, egui::vec2(0.0, 0.0));


        Self {
            fader_list,
            panel_resolution,
            ui_auto_scaller: AutoScaller::new(),
            rect,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        

            egui::CentralPanel::default().show(ctx, |ui| {
                ctx.request_repaint();
                self.rect = ui.max_rect();
                self.panel_resolution = Vec2 { x: self.rect.max.x / 100., y: self.rect.max.y / 100.};

                let mut style = (*ctx.style()).clone();
                style.spacing.slider_width = (self.ui_auto_scaller.get_cell_size().y); // Wider slider
                style.spacing.interact_size.y = (self.ui_auto_scaller.get_cell_size().x / 1.75); // Taller handle
                style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };
    
                ui.set_style(style.clone());


                self.draw_slider_bank(ui);

                //self.draw_ctrl_buttons(ui);

            });

        
    }

    fn draw_slider_bank(&mut self, ui: &mut Ui){
        

        for i in 0..24{           
            

            let local = egui::Rect::from_min_size(self.ui_auto_scaller.get_rect(self.rect, true, i as u8).min, egui::vec2(20.0, 20.0));

            
            
            let response = ui.put(
                local,
                egui::Slider::new(&mut self.fader_list[i].fader_value, 0..=255)
                    .show_value(false)
                    .orientation(egui::SliderOrientation::Vertical),
            );

            if response.changed(){
                self.fader_list[i].fader_value_changed();
            }

            let local = self.ui_auto_scaller.get_rect(self.rect, false, i as u8);

            


            if self.fader_list[i].is_selected == false && ui.put(
                local,
                Button::new(format!("{}", i + 1))
            ).clicked(){
                
                self.fader_list[i].is_selected = true;
            }else if self.fader_list[i].is_selected == true && ui.put(
                local,
                Button::new(format!("{}", i + 1))
            ).clicked(){
                self.fader_list[i].is_selected = false;
            }            

            let frame_rect =self.ui_auto_scaller.get_rect(self.rect, false, i as u8);


            let local = egui::Rect::from_min_size(
                frame_rect.min - Vec2{x:(frame_rect.min.x - frame_rect.max.x) / 2.5 ,y:self.panel_resolution.y * -0.4},
                 egui::vec2(self.panel_resolution.y * 1., 9.)
                );

            

            ui.allocate_rect(local, egui::Sense::hover());


            let mut color = Color32::from_rgb(0, 0, 0);

            if self.fader_list[i].is_selected == true{
                color = Color32::from_rgb(255, 200, 120);
                ui.painter().rect_filled(
                    local,
                    5.0,// corner radius
                    color, // background color
                );
            }   
        }    
        
    }

    fn draw_ctrl_buttons(&mut self, ui: &mut Ui){
        let local = self.ui_auto_scaller.get_ctrl_button(0);

        ui.put(local, Button::new("Test"));
    }
}


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}