pub struct Fader {
    pub fader_value: u8,
    pub previous_fader_value: u8,

    pub is_selected: bool,

    pub fixture_id: Option<u8>,
    pub fixture_component_index: Option<u8>,
}

impl Fader {
    pub fn new(fixture_id: Option<u8>, fixture_component_index: Option<u8>) -> Self {
        Self {
            fader_value: 0,
            previous_fader_value: 0,
            is_selected: false,
            fixture_id,
            fixture_component_index,
        }
    }

    pub fn fader_value_changed(&mut self) {
        if self.is_selected == false && self.fader_value != 0 && self.previous_fader_value != 0 {
        } else if self.is_selected == false && self.fader_value != 0 {
            self.is_selected = true;
        } else if self.is_selected && self.fader_value == 0 {
            self.is_selected = false
        }

        self.previous_fader_value = self.fader_value;
    }
}
