use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread::{self, spawn},
};

use crate::dmx::DmxUniverse;

pub fn start_ws_server(dmx_universe: Arc<Mutex<DmxUniverse>>) {
    let server = TcpListener::bind("127.0.0.1:8000").expect("Couldnt launch server");
    for stream in server.incoming() {
        let uni = dmx_universe.clone();
        thread::spawn(move || {
            let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();
                if msg.is_text() {
                    println!("recieved: {}", msg.to_text().unwrap())
                }
            }
        });
    }
}
