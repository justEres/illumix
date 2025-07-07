use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread::{self},
};

use fixture_lib::universe::Universe;

pub fn start_ws_server(dmx_universe: Arc<Mutex<Universe>>) {
    let server = TcpListener::bind("127.0.0.1:8000").expect("Couldnt launch server");
    for stream in server.incoming() {
        let uni = dmx_universe.clone();
        thread::spawn(move || {
            let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
            websocket
                .send(uni.lock().unwrap().export_to_json().into())
                .expect("Couldnt send universe");
            loop {
                let msg = match websocket.read() {
                    Ok(msg) => msg,
                    Err(_) => {
                        println!("Connection Closed");
                        return;
                    }
                };
                if msg.is_text() {
                    let new_uni = Universe::import_from_json(msg.to_text().unwrap());
                    *uni.lock().unwrap() = new_uni;
                    println!("{}", uni.lock().unwrap().export_to_json());
                }
            }
        });
    }
}
