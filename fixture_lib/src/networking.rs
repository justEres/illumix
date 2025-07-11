use serde::{Deserialize, Serialize};

use crate::{fixture::FixtureComponent, universe::Universe};



#[derive(Serialize,Deserialize,Debug)]
pub struct Packet{
    pub packet_type: PacketType,

}


#[derive(Serialize,Deserialize,Debug)]
pub enum PacketType{
    RequestFullUniverse,
    FullUniverse(Universe),
    FixtureComponentUpdated(FixtureComponentUpdated),
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FixtureComponentUpdated{
    pub component: FixtureComponent,
    pub fixture_id: u8,
}

impl Packet{
    pub fn serialize(self) -> Vec<u8>{
        serde_json::to_vec(&self).expect("Couldnt serialize Packet")
    }

    pub fn deserialize(data: Vec<u8>) -> Packet{
        serde_json::from_slice(&data).expect("Couldnt deserialize Packet")
    }
}