use serde::{Deserialize, Serialize};

use crate::fixture::{self, Fixture};

#[derive(Serialize, Deserialize,Debug)]
pub struct Universe {
    pub fixtures: Vec<Fixture>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            fixtures: Vec::new(),
        }
    }

    pub fn get_fixture_by_id(&self, id: u8) -> Option<&Fixture>{
        for fixture in &self.fixtures{
            if fixture.id == id{
                return Some(&fixture);
            }
        }
        None
    }

    pub fn get_dmx_values(&self) -> [u8; 512] {
        let mut dmx_values = [0u8; 512];
        for fixture in &self.fixtures {
            let fixture_values = fixture.get_dmx_values();
            dmx_values[(fixture.dmx_address - 1) as usize
                ..(fixture.dmx_address as usize - 1 + fixture_values.len())]
                .copy_from_slice(&fixture_values);
        }

        return dmx_values;
    }

    pub fn add_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }

    pub fn export_to_json(&self) -> String {
        serde_json::to_string(&self).expect("Couldnt write to String")
    }

    pub fn import_from_json(json_string: &str) -> Universe {
        serde_json::from_str(json_string).expect("Couldnt parse json string into dmx universe")
    }
}
