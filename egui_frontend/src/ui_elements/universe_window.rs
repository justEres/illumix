use std::sync::Arc;

use eframe::egui::{
    self,
    ahash::{HashMap, HashMapExt},
    mutex::Mutex,
};
use fixture_lib::fixture::FixtureComponent;

use crate::{
    UniverseState, color_picker::ColorPickerWindow, fixture_component_ui::draw_fixture_component,
};

pub struct UniverseWindow {
    universe: Arc<Mutex<UniverseState>>,
    sync_color: HashMap<u8, bool>,
}

impl UniverseWindow {
    pub fn new(ctx: &egui::Context, universe: Arc<Mutex<UniverseState>>) -> UniverseWindow {
        UniverseWindow {
            universe,
            sync_color: HashMap::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Universe").show(ctx, |ui| {
            ui.heading("Fixtures:");

            let mut modified = false;
            for fixture in &mut self.universe.lock().universe.fixtures {
                ui.push_id(fixture.id, |ui| {
                    ui.collapsing(&fixture.name, |ui| {
                        for component in &mut fixture.components {
                            let mut comp = component.clone();
                            draw_fixture_component(ui, &mut comp, fixture.id, &mut self.sync_color);
                            if &comp != component {
                                modified = true;
                            }
                            *component = comp;
                        }
                    });
                });
            }

            self.universe.lock().modified = modified;
        });
    }

    pub fn sync_fixtures_with_color_picker(&mut self, color_picker: &ColorPickerWindow) {
        let color = color_picker.selected_color;

        let mut modified = self.universe.lock().modified;
        for fixture in &mut self.universe.lock().universe.fixtures {
            if *self.sync_color.get(&fixture.id).unwrap_or(&false) {
                for component in &mut fixture.components {
                    match component {
                        FixtureComponent::Color(c) => {
                            if !(c.r == color.r() && c.g == color.g() && c.b == color.b()) {
                                c.r = color.r();
                                c.g = color.g();
                                c.b = color.b();
                                modified = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        self.universe.lock().modified = modified;
    }
}
