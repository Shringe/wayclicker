mod cli;
mod server;

use std::{thread::sleep, time::Duration};

use crate::server::Server;
use clap::Parser;

// fn start_clicker() {
//     let mut autoclicker = false; // Autoclicker toggle variable.
//     let mut clicker = Clicker::new(); // Clicker object.
//     let mut evdevdevice = get_evdev_device(); // Evdev device.
//
//     // Last frame's pressed keys.
//     let mut lastkeys = vec![];
//
//     loop {
//         // Get all currently pressed keys.
//         let keypresses = evdevdevice
//             .get_key_state()
//             .unwrap()
//             .iter()
//             .collect::<Vec<KeyCode>>();
//
//         // Check for specific key presses
//         for k in &keypresses {
//             if k.0 == 62 && !lastkeys.contains(k) {
//                 autoclicker = !autoclicker;
//             }
//         }
//
//         // Update lastkeys.
//         lastkeys = keypresses;
//
//         // Repeatedly click when the autoclicker is on.
//         if autoclicker {
//             clicker.click(0);
//             thread::sleep(time::Duration::from_millis(50));
//         }
//     }
// }

fn start_server() {
    let mut server = Server::default().expect("Failed to get clicker");
    let pause = Duration::from_millis(500);
    sleep(pause);
    let _ = server.click();
}

fn main() {
    let args = cli::Args::parse();

    match args.mode {
        cli::Mode::Server => start_server(),
        cli::Mode::Client => todo!(),
    }
}
