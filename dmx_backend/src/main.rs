use std::{
    fs::{File, OpenOptions},
    io::Write,
    process::Command,
    sync::{Arc, Mutex},
};

use crate::{dmx::DmxPort, frontend_server::get_routes};
use clap::{Arg, Parser};
use gethostname::gethostname;
use libmdns::Responder;
use tracing::info;
use tracing_subscriber;
mod dmx;
mod frontend_server;
mod rgb_hex;
mod server;

use fixture_lib::patching::Patching;

#[derive(Parser, Debug)]
#[command(
    name = "Backend",
    version,
    about = "This serves the egui frontend and opens a websocket server."
)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    #[arg(long, default_value_t = false)]
    headless: bool,
    #[arg(short, long, default_value_t = false)]
    dmx: bool,
    #[arg(short, long, default_value_t = false)]
    artnet: bool,
    #[arg(short, long, default_value_t = false)]
    bonjour: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact() // Pretty, human-readable output
        .with_ansi(true)
        .without_time()
        .with_target(false) // Enable ANSI colors
        .init();

    let args = Args::parse();

    let patching = Patching::load_from_file("patching/patching.json".into());

    let uni = patching.to_universe();

    let uni = Arc::new(Mutex::new(uni));
    info!("Created Dmx Universe.");

    //For normal dmx Backend
    if args.dmx {
        let port = DmxPort::open();
        port.launch_send_thread(uni.clone());
    }

    //For artnet support:
    if args.artnet {
        DmxPort::launch_artnet_send_thread(uni.clone());
    }

    set_ws_path_in_frontend(args.headless, args.port, args.bonjour);
    if !args.headless {
        build_frontend();
        let routes = get_routes(uni.clone());

        let mut _svc;
        if args.bonjour {
            info!(
                "announcing service under http://illumix.local:{}",
                args.port
            );
            _svc = announce_illumix(args.port);
        } else {
            info!(
                "serving frontend with Warp under: {}",
                format!(
                    "http://{}:{}",
                    gethostname().into_string().unwrap(),
                    args.port
                )
            );
        }

        warp::serve(routes).run((([0, 0, 0, 0], args.port))).await
    } else {
        server::start_ws_server(uni).await;
    }
}

fn build_frontend() {
    info!("Building Frontend");
    let status = Command::new("trunk")
        .args(&["build", "--release"])
        .env("RUSTFLAGS", "-Awarnings")
        .current_dir("../egui_frontend")
        .status()
        .expect("Failed to run trunk build");
    info!("Frontend Built");
}

fn set_ws_path_in_frontend(headless: bool, port: u16, bonjour: bool) {
    let mut path = String::new();
    if headless {
        path = String::from("ws://127.0.0.1:8000")
    } else {
        if bonjour {
            path = format!("ws://illumix.local:{}/ws", port);
        } else {
            path = format!("ws://{}:{}/ws", gethostname().into_string().unwrap(), port);
        }
    }
    info!("set frontend websocket path to: {}", path);
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .truncate(true)
        .open("../egui_frontend/src/ws_config.txt")
        .unwrap();
    file.write(path.as_bytes()).unwrap();
}

fn announce_illumix(port: u16) -> libmdns::Service {
    let responder = Responder::new().expect("Failed to create mDNS responder");

    // Advertise the HTTP service under the name "illumix"
    responder.register(
        "_http._tcp".to_owned(),
        "illumix".to_owned(), // this will appear as illumix.local
        port,
        &["path=/"],
    )
}
