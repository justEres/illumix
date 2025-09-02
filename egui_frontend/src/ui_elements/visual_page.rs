use eframe::egui::{
    self, CentralPanel, ColorImage, Image, Layout, Pos2, Rect, Sense, Stroke, TextureHandle,
    TextureOptions, Vec2, include_image,
};
use fixture_lib::{
    fixture::{FixtureComponent, Position},
    universe::{self, Universe},
};

use crate::fixture_component_listener::{ChangeEventManager, ListenerDatabase, SharedState};

const FORUM_IMAGE: &[u8] = include_bytes!("../../assets/forum.png");

struct Fixture {
    fixture_id: u8,
    position: Pos2, // fixed square location (fixture base)
    target: Pos2,   // where the fixture is pointing
}

pub struct VisualPage {
    universe: SharedState<Universe>,
    forum_texture: TextureHandle,
    change_event_manager: SharedState<ChangeEventManager>,
    listener_database: SharedState<ListenerDatabase>,
    fixtures: Vec<Fixture>,
}

impl VisualPage {
    pub fn new(
        universe: SharedState<Universe>,
        ctx: &egui::Context,
        change_event_manager: SharedState<ChangeEventManager>,
        listener_database: SharedState<ListenerDatabase>,
    ) -> Self {
        let forum_image = image::load_from_memory(FORUM_IMAGE).expect("Failed to load image");
        let size = [forum_image.width() as usize, forum_image.height() as usize];
        let rgba = forum_image.to_rgba8();
        let pixels = rgba.into_raw();

        let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);

        let forum_texture = ctx.load_texture("forum_image", color_image, TextureOptions::default());

        // Example: two fixtures at fixed positions
        let fixtures = vec![
            Fixture {
                fixture_id: 25,
                position: Pos2::new(300.0, 420.0),
                target: Pos2::new(300.0, 700.0),
            },
            Fixture {
                fixture_id: 26,
                position: Pos2::new(770.0, 420.0),
                target: Pos2::new(770.0, 700.0),
            },
        ];

        Self {
            change_event_manager,
            listener_database,
            universe,
            forum_texture,
            fixtures,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let available = ui.available_size();
                    let img = Image::new(&self.forum_texture)
                        .max_size(available)
                        .shrink_to_fit()
                        .maintain_aspect_ratio(true);

                    // Actually add the image to the UI
                    let response = ui.add(img);
                    let rect = response.rect;
                    let painter = ui.painter_at(rect);

                    const TILT_EXP: f32 = 2.0;

                    // Use the image area diagonal as a reasonable "max distance" for normalization.
                    let max_dist = (rect.width().powi(2) + rect.height().powi(2)).sqrt();
                    // Before drawing fixtures, compute scale + offset from image coords → screen coords:
                    let tex_size = self.forum_texture.size();
                    let scale_x = rect.width() / tex_size[0] as f32;
                    let scale_y = rect.height() / tex_size[1] as f32;

                    // Before drawing fixtures, compute scale + offset from image coords → screen coords:

                    // draw fixtures on top of the image
                    for (idx, fixture) in self.fixtures.iter_mut().enumerate() {
                        // transform fixture coords from image space → screen space
                        // If your fixture.position/target are already in screen coords, keep as is.
                        // Otherwise, you'll need to map (x,y) from original image size to `rect`.
                        let pos_screen = Pos2::new(
                            rect.min.x + fixture.position.x * scale_x,
                            rect.min.y + fixture.position.y * scale_y,
                        );
                        let target_screen = Pos2::new(
                            rect.min.x + fixture.target.x * scale_x,
                            rect.min.y + fixture.target.y * scale_y,
                        );

                        let scale = 3.0;

                        // Blue square = fixture base
                        let square_size = 10.0 * scale;
                        let square =
                            Rect::from_center_size(pos_screen, egui::Vec2::splat(square_size));
                        painter.rect_filled(square, 4.0, egui::Color32::LIGHT_BLUE);

                        // Red circle = target (draggable)
                        let target_radius = 6.0 * scale;
                        let target_rect = Rect::from_center_size(
                            target_screen,
                            egui::Vec2::splat(target_radius * 2.0),
                        );

                        let id = ui.id().with(idx);
                        let response = ui.interact(target_rect, id, Sense::drag());

                        if response.dragged() {
                            fixture.target.x += response.drag_delta().x / scale_x;
                            fixture.target.y += response.drag_delta().y / scale_y;
                        }

                        painter.circle_filled(target_screen, target_radius, egui::Color32::RED);

                        // Yellow line
                        painter.line_segment(
                            [pos_screen, target_screen],
                            egui::Stroke::new(3.0 * scale, egui::Color32::YELLOW),
                        );

                        // DMX calc (same as before)
                        // --- Pan as angle between points ---
                        let dx = target_screen.x - pos_screen.x;
                        let dy = target_screen.y - pos_screen.y;

                        // atan2(dy, dx) gives radians where 0 is +X axis, CCW positive
                        let angle_rad = dy.atan2(dx);
                        let mut angle_deg = angle_rad.to_degrees();
                        if angle_deg < 0.0 {
                            angle_deg += 360.0;
                        }

                        // Map angle 0..360° -> 0..65535 (16-bit)
                        let pan_dmx =
                            ((angle_deg / 540.0) * 65535.0).round().clamp(0.0, 65535.0) as u16;

                        // --- Tilt as distance, normalized to give finer control at large distances ---
                        let distance = (dx * dx + dy * dy).sqrt();
                        let dist_norm = if max_dist > 0.0 {
                            (distance / max_dist).clamp(0.0, 1.0)
                        } else {
                            0.0
                        };

                        // Inverted mapping: DMX value is larger when close (dist_norm=0),
                        // and smaller when far (dist_norm -> 1). The exponent TILT_EXP
                        // makes changes near the edge smaller (finer control when far).
                        let tilt_norm = (1.0 - dist_norm).powf(TILT_EXP);
                        let tilt_dmx = ((0.5 + tilt_norm * 0.5) * 65535.0)
                            .round()
                            .clamp(0.0, 65535.0) as u16;
                        if response.dragged() {
                            //println!("Fixture #{} -> Pan: {}  Tilt: {}", idx, pan, tilt);
                            let mut component_index = 255;
                            if let Some(fixture_in_uni) =
                                self.universe.borrow().get_fixture_by_id(fixture.fixture_id)
                            {
                                //web_sys::console::log_1(&format!("{:?}",fixture_in_uni).into());
                                for (cid, c) in fixture_in_uni.components.iter().enumerate() {
                                    match c {
                                        fixture_lib::fixture::FixtureComponent::Position(p) => {
                                            component_index = cid;
                                            //web_sys::console::log_1(&format!("component id: {:?}",cid).into());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            let fcomp = FixtureComponent::Position(Position {
                                pan: pan_dmx,
                                tilt: tilt_dmx,
                            });
                            //web_sys::console::log_1(&format!("{}",component_index).into());
                            if component_index != 255 {
                                self.change_event_manager.borrow_mut().create_event(
                                    fixture.fixture_id,
                                    component_index as u8,
                                    fcomp,
                                );
                            }
                        }
                    }
                },
            )
        });
    }
}
