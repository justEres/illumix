use eframe::egui::{Rgba, Slider, Ui, ahash::HashMap, color_picker::color_edit_button_rgba};
use fixture_lib::fixture::FixtureComponent;

pub fn draw_fixture_component(
    ui: &mut Ui,
    component: &mut FixtureComponent,
    fixture_id: u8,
    sync_color: &mut HashMap<u8, bool>,
) {
    match component {
        FixtureComponent::Color(c) => {
            if !sync_color.contains_key(&fixture_id) {
                sync_color.insert(fixture_id, false);
            }
            ui.checkbox(
                sync_color.get_mut(&fixture_id).unwrap_or(&mut false),
                "Sync with color picker",
            );
        }
        FixtureComponent::Dimmer(d) => {
            ui.horizontal(|ui|{
                ui.label("Dimmer: ");
                ui.add(Slider::new(&mut d.intensity, 0..=u8::MAX));
            });
        }
        FixtureComponent::Position(p) => {
            ui.horizontal(|ui|{
                ui.label("Pan: ");
                ui.add(Slider::new(&mut p.pan, 0..=u16::MAX));
            });
            ui.horizontal(|ui|{
                ui.label("Tilt: ");
                ui.add(Slider::new(&mut p.tilt, 0..=u16::MAX));
            });
        }
        FixtureComponent::ColorWheel(c) => {
            ui.horizontal(|ui| {
                ui.label("Color Wheel Index: ");
                ui.add(Slider::new(&mut c.index, 0..=u8::MAX));
            });
        }
        FixtureComponent::CustomValue(c) => {
            ui.horizontal(|ui| {
                ui.label(&c.name);
                ui.add(Slider::new(&mut c.value, 0..=u8::MAX))
            });
        }
        _ => {}
    }
}
