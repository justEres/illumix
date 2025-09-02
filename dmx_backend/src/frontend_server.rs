use std::fs::read_to_string;
use std::sync::Arc;

use crate::server::{PeerMap, handle_message};
use fixture_lib::{
    networking::{Packet, PacketType, universe_update_fixture_component},
    universe::Universe,
};
use futures::{SinkExt, StreamExt};
use tracing::info;
use std::{collections::HashMap, net::SocketAddr, sync::Mutex};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tungstenite::Message;
use uuid::Uuid;
use warp::Filter;

pub fn get_routes(
    universe: Arc<Mutex<Universe>>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let peer_map: PeerMap = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    let peers_filter = warp::any().map(move || peer_map.clone());
    let universe_filter = warp::any().map(move || universe.clone());

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(peers_filter)
        .and(universe_filter)

        .map(|ws: warp::ws::Ws, peers, uni| {
            ws.on_upgrade(move |socket| handle_connection(socket, peers, uni))
        });

    let frontend_dist = "../egui_frontend/dist";
    let index_route = warp::path::end().map(move || {
        warp::reply::html(read_to_string(format!("{}/index.html", frontend_dist)).unwrap())
    });
    let static_route = warp::fs::dir(frontend_dist).with(warp::filters::compression::gzip());

    let routes = ws_route.or(index_route).or(static_route);
    return routes;
}

async fn handle_connection(
    ws_stream: warp::ws::WebSocket,
    peers: PeerMap,
    uni: Arc<Mutex<Universe>>,
) {

    
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = unbounded_channel();

    
    let uuid = Uuid::new_v4();
    info!("New Client: {}",uuid);
    peers.lock().await.insert(uuid, tx);

    // Sending task
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender
                .send(warp::ws::Message::from(tungstenite_to_warp(msg)))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Receiving task
    let recv_task = {
        let peers = peers.clone();
        let uni = uni.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = ws_receiver.next().await {
                if msg.is_text() || msg.is_binary() {
                    handle_message(warp_to_tungstenite(msg), peers.clone(), uuid, uni.clone())
                        .await;
                }
            }
        })
    };

    tokio::select! {
        _ = send_task => {}
        _ = recv_task => {}
    }

    peers.lock().await.remove(&uuid);
    info!("Client disconnected: {}", uuid);
}

fn tungstenite_to_warp(msg: tungstenite::Message) -> warp::ws::Message {
    match msg {
        tungstenite::Message::Text(s) => warp::ws::Message::text(s.to_string()),
        tungstenite::Message::Binary(b) => warp::ws::Message::binary(b),
        tungstenite::Message::Ping(v) => warp::ws::Message::ping(v),
        tungstenite::Message::Pong(v) => warp::ws::Message::pong(v),
        tungstenite::Message::Close(c) => warp::ws::Message::close(),
        tungstenite::Message::Frame(_) => warp::ws::Message::close(),
    }
}

fn warp_to_tungstenite(msg: warp::ws::Message) -> tungstenite::Message {
    if msg.is_text() {
        tungstenite::Message::Text(tungstenite::Utf8Bytes::from(msg.to_str().unwrap()))
    } else if msg.is_binary() {
        tungstenite::Message::Binary(msg.into_bytes())
    } else if msg.is_ping() {
        tungstenite::Message::Ping(tungstenite::Bytes::copy_from_slice(msg.as_bytes()))
    } else if msg.is_pong() {
        tungstenite::Message::Pong(tungstenite::Bytes::copy_from_slice(msg.as_bytes()))
    } else if msg.is_close() {
        tungstenite::Message::Close(None)
    } else {
        tungstenite::Message::Close(None)
    }
}
