use std::sync::Arc;

use eframe::egui::mutex::Mutex;
use fixture_lib::{networking::Packet, universe::Universe};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{js_sys::{self, ArrayBuffer, Uint8Array}, ErrorEvent, Event, MessageEvent};

use crate::UniverseState;

pub fn open_websocket(uni: Arc<Mutex<UniverseState>>) -> web_sys::WebSocket {
    let ws = web_sys::WebSocket::new("ws://127.0.0.1:8000").unwrap();

    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(data) = e.data().dyn_into::<ArrayBuffer>() {
            let packet = Packet::deserialize(Uint8Array::new(&data).to_vec());
            handle_packet(packet, uni.clone());

            //web_sys::console::log_1(&format!("Recieved: {}", txt).into());
        } else {
            web_sys::console::log_1(&"Couldnt parse data".into());
        }
    }) as Box<dyn FnMut(_)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    // Handle error event
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        web_sys::console::log_1(&format!("Error: {:?}", e).into());
    }) as Box<dyn FnMut(_)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    // Handle open event
    let ws_clone = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_e: Event| {
        web_sys::console::log_1(&"WebSocket connection opened!".into());
        let packet = Packet{
            packet_type: fixture_lib::networking::PacketType::RequestFullUniverse,
        };
        

        // Send a message once opened
        if let Err(err) = ws_clone.send_with_u8_array(&packet.serialize()) {
            web_sys::console::log_1(&format!("Error sending message: {:?}", err).into());
        }
    }) as Box<dyn FnMut(_)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
    ws
}


pub fn handle_packet(packet: Packet, uni: Arc<Mutex<UniverseState>>){
    let packet_text = format!("{:?}",packet);
    web_sys::console::log_1(&packet_text.into());
}