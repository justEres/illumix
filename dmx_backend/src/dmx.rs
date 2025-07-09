use std::{
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use artnet_protocol::{ArtCommand, Output};
use fixture_lib::universe::{self, Universe};
use serialport::SerialPort;
use tracing::info;

pub struct DmxPort {
    serial_port: Box<dyn SerialPort>,
}

impl DmxPort {
    pub fn open() -> DmxPort {
        info!("Connecting to serial device...");
        let port = serialport::new("/dev/ttyUSB0", 250_000)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::Two)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Couldnt Open Serial Port");
        info!("Serial device connected.");
        return DmxPort { serial_port: port };
    }

    pub fn launch_send_thread(mut self, universe: Arc<Mutex<Universe>>) {
        thread::spawn(move || {
            info!("Dmx tread started.");
            let port = &mut self.serial_port;

            loop {
                // Send DMX Break
                port.set_break().ok();
                thread::sleep(Duration::from_micros(120));
                port.clear_break().ok();
                thread::sleep(Duration::from_micros(12));
                // Copy data from shared Dmx Universe
                let mut new_channels: [u8; 513] = [0; 513];
                {
                    new_channels[1..].copy_from_slice(&universe.lock().unwrap().get_dmx_values());
                }
                // write to port
                port.write_all(&new_channels)
                    .expect("Failed to write DMX data");
                port.flush().ok();
                thread::sleep(Duration::from_millis(25)); // ~40 FPS
            }
        });
    }

    pub fn launch_artnet_send_thread(universe: Arc<Mutex<Universe>>) {
        thread::spawn(move || {
            let socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();
            info!("Created Socket for ArtNet");

            loop {
                let command = ArtCommand::Output(Output {
                    data: Vec::from(universe.lock().unwrap().get_dmx_values()).into(),
                    ..Output::default()
                });

                let buffer = command.write_to_buffer().expect("Failed to serialize");

                socket.send_to(&buffer, ("127.0.0.1", 6454)).unwrap();
                //info!("Sent Art-Net DMX");
            }
        });
    }
}
