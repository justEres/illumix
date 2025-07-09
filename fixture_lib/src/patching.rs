//Darth Fader

use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{
    fixture::{self, FixtureComponent},
    universe::Universe,
};

#[derive(Serialize, Deserialize)]
pub struct FixturePreset {
    pub name: String,
    pub components: Vec<FixtureComponent>,
}

#[derive(Serialize, Deserialize)]
pub struct Fixture {
    pub dmx_address: u16,
    pub id: u8,
    pub fixture_preset: String, //path to json file
}

#[derive(Serialize, Deserialize)]
pub struct Patching {
    pub fixtures: Vec<Fixture>,
}

impl Patching {
    pub fn new() -> Self {
        Patching {
            fixtures: Vec::new(),
        }
    }

    pub fn to_universe(&self) -> Universe {
        let mut fixtures = Vec::new();
        for f in &self.fixtures {
            fixtures.push(f.resolve());
        }
        Universe { fixtures }
    }

    pub fn load_from_file(path: String) -> Self {
        let patching_file = File::open(&path).expect(&format!(
            "Couldnt load Patching: File doesnt Exist: {}",
            &path
        ));
        serde_json::from_reader(patching_file).expect("Couldnt parse Json")
    }

    pub fn store_to_file(&self, path: String) {
        let patching_file = File::create(&path).expect(&format!("Couldnt create File: {}", &path));
        serde_json::to_writer(patching_file, self).expect("Couldnt write to file");
    }
}

impl Fixture {
    fn resolve(&self) -> fixture::Fixture {
        let preset_file = File::open(&self.fixture_preset).expect(&format!(
            "Couldnt load Preset: File doesnt Exist: {}",
            self.fixture_preset
        ));
        let preset: FixturePreset =
            serde_json::from_reader(preset_file).expect("Couldnt parse Json");

        fixture::Fixture {
            id: self.id,
            name: preset.name,
            dmx_address: self.dmx_address,
            components: preset.components,
        }
    }
}

impl FixturePreset {
    pub fn store_to_file(&self, path: String) {
        let fixture_file = File::create(&path).expect(&format!("Couldnt create File: {}", &path));
        serde_json::to_writer(fixture_file, self).expect("Couldnt write to file");
    }
}
