mod cli;
mod server;

use std::time::Duration;

use crate::server::Server;
use clap::Parser;
use evdev::{Device, KeyCode};

fn main() {
    let args = cli::Args::parse();

    match args.mode {
        cli::Mode::Server {
            device,
            interval,
            keybind,
        } => {
            let listenor = Device::open(device).unwrap();
            let interval = Duration::from_millis(interval);
            let mut server = Server::new(listenor, interval, keybind, args.debug)
                .expect("Failed to get create server");
            server.run();
        }

        cli::Mode::Client => todo!(),

        cli::Mode::List => {
            for (path, device) in evdev::enumerate() {
                println!("{:?}: {}", path, device.name().unwrap_or("<error>"));
            }
        }
    }
}
