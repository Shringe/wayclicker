mod cli;
mod hotkey;
mod server;

use clap::Parser;

use crate::server::Server;

fn main() {
    env_logger::init();
    let args = cli::Args::parse();
    let mut server = Server::new().expect("Failed to initialize server");
    server.run();
}
