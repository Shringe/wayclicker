mod cli;
mod hotkey;
mod server;

use crate::server::Server;
use clap::Parser;
use std::time::Duration;

fn main() {
    let args = cli::Args::parse();

    match args.mode {
        cli::Mode::Start {
            device,
            interval,
            modifiers,
            keybind,
        } => {
            env_logger::init();
            let listenor = evdev::Device::open(device)
                .expect("Failed to open evdev device for listening to the hotkey");
            let interval = Duration::from_millis(interval);
            let mut server = Server::new(listenor, interval, modifiers, keybind)
                .expect("Failed to create server");
            server.run();
        }

        cli::Mode::List => {
            for (path, device) in evdev::enumerate() {
                println!("{:?}: {}", path, device.name().unwrap_or("<error>"));
            }
        }
    }
}
