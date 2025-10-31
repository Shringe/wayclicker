mod cli;
mod hotkey;
mod server;

use crate::server::Server;
use clap::Parser;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    match args.mode {
        cli::Mode::Start {
            device,
            modifiers,
            keybind,
        } => {
            env_logger::init();
            let listener = evdev::Device::open(device)
                .expect("Failed to open evdev device for listening to the hotkey");
            let mut server = Server::new(
                listener,
                modifiers,
                keybind,
                PathBuf::from("/tmp/wayclicker.sock"),
            )
            .expect("Failed to create server");

            server
                .listen_control_socket()
                .await
                .expect("Failed to start control socket");
            server.wait_for_shutdown().await;
            server.run().await;
        }

        cli::Mode::List => {
            for (path, device) in evdev::enumerate() {
                println!("{:?}: {}", path, device.name().unwrap_or("<error>"));
            }
        }
    }
}
