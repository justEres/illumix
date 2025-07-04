use std::{io::{stdin, stdout, Write}, sync::{Arc, Mutex}};

use crate::dmx::{DmxPort};
use tracing::info;
use tracing_subscriber;
mod dmx;
mod rgb_hex;
mod server;

use fixture_lib::{fixture::{Color, Dimmer, Fixture}, universe::Universe};

fn main() {
    tracing_subscriber::fmt()
        .compact() // Pretty, human-readable output
        .with_ansi(true)
        .without_time()
        .with_target(false) // Enable ANSI colors
        .init();

    let mut uni = Universe::new();
    let mut f = Fixture::new(0, 1, "Eurolite Led".into());
    f.add_component(fixture_lib::fixture::FixtureComponent::Color(Color{r:0,g:0,b:255}));
    f.add_component(fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer { intensity: 255 }));
    uni.add_fixture(f);

    let uni = Arc::new(Mutex::new(uni));
    info!("Created Dmx Universe.");

    let port = DmxPort::open();
    port.launch_send_thread(uni.clone());

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
