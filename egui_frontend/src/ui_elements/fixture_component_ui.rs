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
            ui.add(Slider::new(&mut d.intensity, 0..=255));
        }
        _ => {}
    }
}
