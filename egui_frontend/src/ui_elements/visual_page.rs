use eframe::egui::{
    self, CentralPanel, ColorImage, Image, Layout, TextureHandle, TextureOptions, include_image,
    load::TexturePoll,
};
use fixture_lib::universe::{self, Universe};

use crate::fixture_component_listener::SharedState;

const FORUM_IMAGE: &[u8] = include_bytes!("../../assets/forum.png");

pub struct VisualPage {
    universe: SharedState<Universe>,
    forum_texture: TextureHandle,
}

impl VisualPage {
    pub fn new(universe: SharedState<Universe>, ctx: &egui::Context) -> Self {
        let forum_image = image::load_from_memory(FORUM_IMAGE).expect("Failed to load image");
        let size = [forum_image.width() as usize, forum_image.height() as usize];
        let rgba = forum_image.to_rgba8();
        let pixels = rgba.into_raw();

        let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);

        let forum_texture = ctx.load_texture("forum_image", color_image, TextureOptions::default());

        Self {
            universe,
            forum_texture,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Visual Page");
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.add(
                        Image::new(&self.forum_texture)
                            .max_size(ui.available_size())
                            .shrink_to_fit()
                            .maintain_aspect_ratio(true),
                    );
                },
            )
        });
    }
}
