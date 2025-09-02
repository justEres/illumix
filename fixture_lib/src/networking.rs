use serde::{Deserialize, Serialize};

use crate::{fixture::FixtureComponent, universe::Universe};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub packet_type: PacketType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    RequestFullUniverse,
    FullUniverse(Universe),
    FixtureComponentUpdated(FixtureComponentUpdated),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixtureComponentUpdated {
    pub component: FixtureComponent,
    pub fixture_id: u8,
    pub component_index: u8,
}

pub fn universe_update_fixture_component(universe: &mut Universe, packet: FixtureComponentUpdated) {
    if let Some(fixture) = universe.get_fixture_by_id_mut(packet.fixture_id) {
        fixture.components[packet.component_index as usize] = packet.component;
    }
}

impl Packet {
    pub fn serialize(self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("Couldnt serialize Packet")
    }

    pub fn deserialize(data: Vec<u8>) -> Packet {
        serde_json::from_slice(&data).expect("Couldnt deserialize Packet")
    }
}
