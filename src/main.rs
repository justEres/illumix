use std::{io::Write, thread, time::Duration};

use crate::dmx::{DmxPort, DmxUniverse};

mod dmx;

fn main() {
    let uni = DmxUniverse::new();

    let port = DmxPort::open();
    port.launch_send_thread(uni.clone());
    dbg!("started init");
    DmxUniverse::set_channel(uni.clone(), 0, 255);
    DmxUniverse::set_channel(uni.clone(), 1, 100);
    DmxUniverse::set_channel(uni.clone(), 2, 0);
    DmxUniverse::set_channel(uni.clone(), 3, 255);

    thread::sleep(Duration::from_secs(1));
    DmxUniverse::set_channel(uni.clone(), 0, 0);
    thread::sleep(Duration::from_secs(1));
    DmxUniverse::set_channel(uni.clone(), 1, 255);
    thread::sleep(Duration::from_secs(1));
    DmxUniverse::set_channel(uni.clone(), 2, 255);
}
