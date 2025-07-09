use std::{fs::File, io::Read, sync::{Arc, Mutex}};

use crate::dmx::DmxPort;
use tracing::info;
use tracing_subscriber;
mod dmx;
mod rgb_hex;
mod server;

use fixture_lib::{
    fixture::{Color, Dimmer, Fixture},
    universe::Universe,
};

fn main() {
    tracing_subscriber::fmt()
        .compact() // Pretty, human-readable output
        .with_ansi(true)
        .without_time()
        .with_target(false) // Enable ANSI colors
        .init();



    let mut patching = File::open("patching.json").expect("Couldnt open file");
    let mut contents = String::new();
    patching.read_to_string(&mut contents).unwrap();
    let mut uni = Universe::import_from_json(&contents);

    /* let mut uni = Universe::new();
    let mut f = Fixture::new(0, 1, "Eurolite Led".into());
    f.add_component(fixture_lib::fixture::FixtureComponent::Color(Color {
        r: 0,
        g: 0,
        b: 255,
    }));
    f.add_component(fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer {
        intensity: 255,
    }));
    uni.add_fixture(f); */

    /* let mut testf = Fixture::new(1, 400, "Käsekuchen".into());
    testf.add_component(fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer { intensity: 3 }));
    testf.add_component(fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer { intensity: 40 }));
    testf.add_component(fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer { intensity: 205}));
    testf.add_component(fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer { intensity: 50 }));

    uni.add_fixture(testf); */

    let uni = Arc::new(Mutex::new(uni));
    info!("Created Dmx Universe.");

    //let port = DmxPort::open();

    //For normal dmx Backend
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
