mod hotkey;
mod server;

use crate::server::Server;

fn main() {
    env_logger::init();
    let mut server = Server::new().expect("Failed to initialize server");
    server.run();
}
