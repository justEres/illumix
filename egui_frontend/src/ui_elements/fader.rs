pub struct Fader {
    pub pos_x: f32,
    pub pos_y: f32,

    pub fader_value: u8,

    pub fixture_id: Option<u8>,
    pub fixture_component_index: Option<u8>,
}

impl Fader {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        fixture_id: Option<u8>,
        fixture_component_index: Option<u8>,
    ) -> Self {
        Self {
            pos_x,
            pos_y,

            fader_value: 0,

            fixture_id,
            fixture_component_index,
        }
    }
}
