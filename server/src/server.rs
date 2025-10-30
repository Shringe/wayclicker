use std::{error::Error, thread, time::Duration};

// Uinput is used for send the inputs
use uinput::event::controller::Mouse;

// Evdev is used for detecting keyboard input globally
use evdev::KeyCode;

use crate::hotkey::HotKey;

pub struct Server {
    clicker: uinput::device::Device,
    hotkey: HotKey,
    interval: Duration,
    /// Whether or not the daemon listens for hotkeys and clicks
    enabled: bool,
}

impl Server {
    /// Creates the server and clicker
    pub fn new(
        listenor: evdev::Device,
        interval: Duration,
        modifiers: String,
        keybind: KeyCode,
    ) -> Result<Self, Box<dyn Error>> {
        let clicker = uinput::default()?
            .name("device")?
            .event(Mouse::Left)?
            .create()?;

        let hotkey = HotKey::new(listenor, modifiers, keybind);
        Ok(Self {
            clicker,
            hotkey,
            interval,
            enabled: true,
        })
    }

    /// Runs the server loop
    pub fn run(&mut self) {
        log::info!("Server ready");
        loop {
            if self.enabled {
                let active = self
                    .hotkey
                    .is_active()
                    .expect("Failed to determine if the hotkey is active");

                if active {
                    if let Err(e) = self.click() {
                        log::error!("Encountered an error while trying to click: {}", e);
                    }
                }
            }

            thread::sleep(self.interval);
        }
    }

    /// Sends a left click
    fn click(&mut self) -> Result<(), Box<dyn Error>> {
        log::debug!("Sending a left click");
        self.clicker.send(Mouse::Left, 1)?;
        self.clicker.send(Mouse::Left, 0)?;
        self.clicker.synchronize()?;
        Ok(())
    }
}
