use clap::{Parser, Subcommand};
use evdev::KeyCode;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// What mode to run the program in
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Runs the autoclicker. The default hotkey to toggle the clicker is F5. May need to run as root
    Start {
        /// Path of the device to listen to hotkeys from
        #[arg(long)]
        device: PathBuf,

        /// Time to sleep in between clicks in milliseconds
        #[arg(long, default_value_t = 50)]
        interval: u64,

        /// Modifiers for the keybind. Can be empty. Use - to define optional alternatives to the
        /// modifier, like to allow both left/right control instead of just one. Use + to add
        /// modifiers, incase you want both shift+ctrl for example.
        #[arg(
            long,
            value_name = "KEY_RIGHTCTRL-KEY_LEFTCTRL+KEY_RIGHTSHIFT-KEY_LEFTSHIFT"
        )]
        modifiers: String,

        /// Keybind to toggle the hotkey. Should be prefixed with KEY_
        #[arg(long, value_name = "KEY_F8")]
        keybind: KeyCode,
    },
    /// Lists input devices. May need root to see all devices.
    List,
}
