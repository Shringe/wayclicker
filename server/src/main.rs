mod hotkey;
mod server;

use crate::server::Server;

fn main() {
    env_logger::init();
    let mut server = Server::default();
    server.run();
}
