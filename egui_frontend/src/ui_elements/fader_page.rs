use eframe::egui::Color32;
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window};

pub struct FaderPage{

}

impl FaderPage{
    pub fn new(ctx: &egui::Context) -> Self {
        Self{

        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        ctx.request_repaint();
        egui::Window::new("Fader Page").show(ctx, |ui|{
            ui.add(egui::Slider::new(&mut 50., 0.0..=100.0).text("My value"));
        });
    }

}