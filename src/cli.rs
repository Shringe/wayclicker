use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Runs the autoclicker. The default hotkey to toggle the clicker is F5. May need to run as root
    Server {
        /// Path of the device to listen to hotkeys from
        #[arg(long)]
        device: PathBuf,

        /// Time to sleep in between clicks
        #[arg(long, default_value_t = 50)]
        interval: u64,
    },
    Client,
    /// Lists input devices. May need root to see all devices.
    List,
}
