
use eframe::egui::{Color32, Pos2, Rect, Style};
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window,Ui, Button};

use crate::fader_page::fader::Fader;

mod fader;




pub struct FaderPage{
    //fader_list: [fader::Fader; 24]
    test_fader: fader::Fader,

    //Styling
    
    panel_resolution: Vec2,
}

impl FaderPage {
    pub fn new(ctx: &egui::Context) -> Self{

        let panel_resolution = Vec2 { x: 0., y: 0. };

        

        Self { 
            test_fader: Fader::new(10., 10., None, None),
            panel_resolution,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context){
        
        CentralPanel::default().show(ctx, |ui| {

            self.panel_resolution = Vec2 { x: ui.min_size().x, y: ui.min_size().y };

    
            /// Styling
            
            let mut style = (*ctx.style()).clone();
            style.spacing.slider_width = ((self.panel_resolution.y / 100.) * 37.5); // Wider slider
            style.spacing.interact_size.y = ((self.panel_resolution.y / 100.) * 6.); // Taller handle
            style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };
            
            
            ui.set_style(style.clone());

             
            let widget_rect = Rect::from_min_size(Pos2 { x: self.test_fader.pos_x, y: self.test_fader.pos_y }, Vec2 { x: 20., y: 20. });
            ui.put(
                widget_rect,
                egui::Slider::new(&mut self.test_fader.fader_value, 0..=255)
                    .show_value(false)
                    .orientation(egui::SliderOrientation::Vertical), //.handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 1.5 })
            );

        });
    }


}

