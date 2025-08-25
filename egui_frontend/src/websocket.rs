use fixture_lib::{
    networking::{Packet, PacketType},
    universe::Universe,
};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{
    ErrorEvent, Event, MessageEvent, WebSocket,
    js_sys::{ArrayBuffer, Error, Uint8Array},
};

use crate::fixture_component_listener::{ListenerDatabase, SharedState};

pub fn open_websocket(
    uni: SharedState<Universe>,
    listener_database: SharedState<ListenerDatabase>,
) -> Result<web_sys::WebSocket, Error> {
    let ws = web_sys::WebSocket::new("ws://127.0.0.1:8000")?;

    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(data) = e.data().dyn_into::<ArrayBuffer>() {
            let packet = Packet::deserialize(Uint8Array::new(&data).to_vec());
            handle_packet(packet, uni.clone(), listener_database.clone());

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
        let packet = Packet {
            packet_type: fixture_lib::networking::PacketType::RequestFullUniverse,
        };

        // Send a message once opened
        if let Err(err) = ws_clone.send_with_u8_array(&packet.serialize()) {
            web_sys::console::log_1(&format!("Error sending message: {:?}", err).into());
        }
    }) as Box<dyn FnMut(_)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
    Ok(ws)
}

pub fn handle_packet(
    packet: Packet,
    uni: SharedState<Universe>,
    listener_database: SharedState<ListenerDatabase>,
) {
    match packet.packet_type {
        PacketType::RequestFullUniverse => {
            web_sys::console::log_1(
                &"got RequestFullUniverse Packet, which is only for the server"
                    .to_string()
                    .into(),
            );
        }
        PacketType::FullUniverse(universe) => {
            *uni.borrow_mut() = universe;
        }
        PacketType::FixtureComponentUpdated(fixture_component_updated) => {
            //web_sys::console::log_1(&"got fixture update packet".into());
            listener_database.borrow().notify(
                fixture_component_updated.fixture_id,
                fixture_component_updated.component_index,
                fixture_component_updated.component,
            );
        }
    }

    //let packet_text = format!("{:?}",packet);
    //web_sys::console::log_1(&packet_text.into());
}

pub fn send_packet(ws: WebSocket, packet: Packet) {
    ws.send_with_u8_array(&packet.serialize())
        .expect("Couldnt send packet.");
}
