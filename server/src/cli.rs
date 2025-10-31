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

        /// Modifiers for the keybind. Can be empty. Use - to define optional alternatives to the
        /// modifier, like to allow both left/right control instead of just one. Use + to add
        /// modifiers, incase you want both shift+ctrl for example.
        #[arg(
            long,
            value_name = "KEY_RIGHTCTRL-KEY_LEFTCTRL+KEY_RIGHTSHIFT-KEY_LEFTSHIFT",
            default_value = ""
        )]
        modifiers: String,

        /// Keybind to toggle the hotkey. Should be prefixed with KEY_
        #[arg(long, default_value = "KEY_F8")]
        keybind: KeyCode,

        /// Path of the unix socket to use. Not recommended to change, as it can confuse clients
        #[arg(long, default_value = "/tmp/wayclicker.sock")]
        socket_path: PathBuf,

        /// The group of users who can configure the autoclicker. For added security, it is
        /// recommended to change this to your personal user, or to create a group called
        /// "wayclicker" for this
        #[arg(long, default_value = "users")]
        socket_group: String,

        /// The rate at which the server checks if the hotkey has been pressed. Setting this too
        /// low can increase background cpu usage
        #[arg(long, default_value_t = 50)]
        hotkey_poll_interval_ms: u64,
    },
    /// Lists input devices. May need root to see all devices.
    List,
}
