mod cli;
mod gui;
mod server;

use std::time::Duration;

use crate::server::Server;
use clap::Parser;
use evdev::Device;

fn main() {
    env_logger::init();

    let args = cli::Args::parse();
    log::debug!("Cli args: {:?}", args);

    match args.mode {
        cli::Mode::Server {
            device,
            interval,
            modifiers,
            keybind,
        } => {
            let listenor = Device::open(device).unwrap();
            let interval = Duration::from_millis(interval);
            let mut server = Server::new(listenor, interval, modifiers, keybind)
                .expect("Failed to get create server");
            server.run();
        }

        cli::Mode::Client => {
            gui::main();
        }

        cli::Mode::List => {
            for (path, device) in evdev::enumerate() {
                println!("{:?}: {}", path, device.name().unwrap_or("<error>"));
            }
        }
    }
}
