mod hotkey;
mod server;

use crate::server::Server;

fn main() {
    let mut server = Server::default();
    server.run();
}
