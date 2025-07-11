use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread::{self},
};

use fixture_lib::{fixture::FixtureComponent, networking::{Packet, PacketType}, universe::Universe};
use tracing::warn;
use tungstenite::WebSocket;

pub fn start_ws_server(dmx_universe: Arc<Mutex<Universe>>) {
    let server = TcpListener::bind("127.0.0.1:8000").expect("Couldnt launch server");
    for stream in server.incoming() {
        let uni = dmx_universe.clone();
        thread::spawn(move || {
            let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
            loop {
                let msg = match websocket.read() {
                    Ok(msg) => msg,
                    Err(_) => {
                        println!("Connection Closed");
                        return;
                    }
                };
                
                let packet = Packet::deserialize(msg.into_data().to_vec());
                handle_packet(packet, &mut websocket, uni.clone());
            }
        });
    }
}

fn handle_packet(packet: Packet,ws: &mut WebSocket<TcpStream>,universe: Arc<Mutex<Universe>>){
    match packet.packet_type {
        PacketType::RequestFullUniverse => {
            let universe = universe.lock().unwrap();
            let packet = Packet{packet_type: PacketType::FullUniverse(universe.clone())};
            ws.send(packet.serialize().into()).unwrap();
        }
        PacketType::FixtureComponentUpdated(fcu) => {
            let mut universe = universe.lock().unwrap();
            let mut fixture = universe.get_fixture_by_id_mut(fcu.fixture_id).unwrap();
            fixture.components.iter_mut().for_each(|c| {
                if std::mem::discriminant(c) == std::mem::discriminant(&fcu.component){
                    *c = fcu.component.clone();
                }
            });
        }
        _ => {
            warn!("Server got unimplemented Packet {:?}",packet);
        }
    }
}
