use std::{
    collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}, thread::{self}
};

use fixture_lib::{ networking::{universe_update_fixture_component, Packet, PacketType}, universe::Universe};

use futures::{SinkExt, StreamExt};
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn};

use tokio::{net::{TcpListener, TcpStream}, sync::mpsc::{unbounded_channel, UnboundedSender}};
use tungstenite::Message;
use uuid::Uuid;


type Tx = UnboundedSender<Message>;
type PeerMap = Arc<tokio::sync::Mutex<HashMap<Uuid, Tx>>>;


pub async fn start_ws_server(uni: Arc<Mutex<Universe>>){
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    info!("Websocket server running on ws://{}",addr);

    let peers: PeerMap = Arc::new(tokio::sync::Mutex::new(HashMap::new()));

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_map = peers.clone();

        tokio::spawn(handle_connection(peer_map, stream, addr,uni.clone()));
    }
}

async fn handle_connection(peers: PeerMap, stream: TcpStream, addr: SocketAddr,uni: Arc<Mutex<Universe>>) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket error: {}", e);
            return;
        }
    };
    
    info!("New client: {}", addr);
    let peers_clone = peers.clone();

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let (tx, mut rx) = unbounded_channel();

        let uuid = Uuid::new_v4();
        peers.lock().await.insert(uuid, tx);

    let send_task = tokio::spawn(async move{
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err(){
                break;
            }
        }
    });
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if msg.is_text() || msg.is_binary() {
                handle_message(msg, peers.clone(), uuid, uni.clone()).await;
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    info!("Client disconnected: {}", addr);
    peers_clone.lock().await.remove(&uuid);

}


async fn broadcast(uuid: &Uuid, peers: PeerMap, message: &Message){
    let peers = peers.lock().await;
    for (id, tx) in peers.iter() {
        if *id != *uuid {
            let _ = tx.send(message.clone());
        }
    }
}


async fn handle_message(msg: Message, peers: PeerMap, uuid: Uuid, universe: Arc<Mutex<Universe>>) {
    let peer_map = peers.lock().await;
    let tx = peer_map.get(&uuid).unwrap();
    let packet = Packet::deserialize(msg.into_data().to_vec());

    match &packet.packet_type {
        PacketType::RequestFullUniverse => {
            let universe = universe.lock().unwrap();
            let packet = Packet{packet_type: PacketType::FullUniverse(universe.clone())};
            tx.send(packet.serialize().into()).unwrap();
        },
        PacketType::FixtureComponentUpdated(fcu) => {
            let mut universe = universe.lock().unwrap();
            universe_update_fixture_component(&mut universe, fcu.clone());
            broadcast(&uuid, peers.clone(), &packet.serialize().into());
        }
        _ => {
            warn!("Server got unimplemented Packet {:?}",packet);
        }
    }
}


