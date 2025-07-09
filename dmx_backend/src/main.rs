use std::{fs::File, io::Read, sync::{Arc, Mutex}};

use crate::dmx::DmxPort;
use tracing::info;
use tracing_subscriber;
mod dmx;
mod rgb_hex;
mod server;

use fixture_lib::{
    fixture::{Color, Dimmer, Fixture}, patching::Patching, universe::Universe
};

fn main() {
    tracing_subscriber::fmt()
        .compact() // Pretty, human-readable output
        .with_ansi(true)
        .without_time()
        .with_target(false) // Enable ANSI colors
        .init();



    let mut patching = Patching::load_from_file("patching/patching.json".into());
    
    let mut uni = patching.to_universe();

    

    let uni = Arc::new(Mutex::new(uni));
    info!("Created Dmx Universe.");

    

    //For normal dmx Backend
    //let port = DmxPort::open();
    //port.launch_send_thread(uni.clone());

    //For artnet support:
    DmxPort::launch_artnet_send_thread(uni.clone());

    /* loop {
        println!("Enter color code:");
        print!("#");
        stdout().flush().unwrap();
        let mut input = String::from("#");
        stdin().read_line(&mut input).unwrap();

        let color = rgb_hex::hex_to_rgb(&input);
        DmxUniverse::set_channel(uni.clone(), 0, color.r);
        DmxUniverse::set_channel(uni.clone(), 1, color.g);
        DmxUniverse::set_channel(uni.clone(), 2, color.b);
        DmxUniverse::set_channel(uni.clone(), 3, 255);
        info!("set color to: {:?}", color)
    } */

    server::start_ws_server(uni);
}
