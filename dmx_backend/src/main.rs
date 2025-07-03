use std::io::{Write, stdin, stdout};

use crate::dmx::{DmxPort, DmxUniverse};
use tracing::info;
use tracing_subscriber;
mod dmx;
mod rgb_hex;

fn main() {
    tracing_subscriber::fmt()
        .compact() // Pretty, human-readable output
        .with_ansi(true)
        .without_time()
        .with_target(false) // Enable ANSI colors
        .init();

    let uni = DmxUniverse::new();
    info!("Created Dmx Universe.");

    let port = DmxPort::open();
    port.launch_send_thread(uni.clone());

    loop {
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
    }
}
