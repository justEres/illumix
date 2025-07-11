use serde::{Deserialize, Serialize};

use crate::universe::Universe;



#[derive(Serialize,Deserialize)]
pub struct Packet{
    packet_type: PacketType,

}


#[derive(Serialize,Deserialize)]
pub enum PacketType{
    RequestFullUniverse,
    FullUniverse(Universe)
}


impl Packet{
    pub fn serialize(self) -> Vec<u8>{
        serde_json::to_vec(&self).expect("Couldnt serialize Packet")
    }

    pub fn deserialize(data: Vec<u8>) -> Packet{
        serde_json::from_slice(&data).expect("Couldnt deserialize Packet")
    }
}