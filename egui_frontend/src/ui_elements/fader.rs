pub struct Fader {

    pub fader_value: u8,

    pub fixture_id: Option<u8>,
    pub fixture_component_index: Option<u8>,
}

impl Fader {
    pub fn new(
        fixture_id: Option<u8>,
        fixture_component_index: Option<u8>,
    ) -> Self {
        Self {

            fader_value: 0,

            fixture_id,
            fixture_component_index,
        }
    }
}
