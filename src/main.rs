use std::{io::Write, thread, time::Duration};

fn main() {
    println!("Starting DMX color fade...");

    let port = serialport::new("/dev/ttyUSB0", 250_000)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::Two)
        .flow_control(serialport::FlowControl::None)
        .timeout(Duration::from_millis(10));

    match port.open() {
        Ok(mut port) => {
            let mut dmx_data = [0u8; 513];
            dmx_data[0] = 0; // Start code
            dmx_data[4] = 255; // Master Dimmer FULL
            dmx_data[5] = 0;   // Effects OFF

            let mut red: u8 = 255;
            let mut green: u8 = 0;
            let mut blue: u8 = 0;

            loop {
                dmx_data[1] = red;
                dmx_data[2] = green;
                dmx_data[3] = blue;

                // Send DMX Break
                port.set_break().ok();
                thread::sleep(Duration::from_micros(120));
                port.clear_break().ok();
                thread::sleep(Duration::from_micros(12));

                // Send DMX Frame
                port.write_all(&dmx_data).expect("Failed to write DMX data");
                port.flush().ok();

                thread::sleep(Duration::from_millis(25)); // ~40 FPS
            }
        }
        Err(e) => {
            eprintln!("Failed to open port: {}", e);
        }
    }
}
