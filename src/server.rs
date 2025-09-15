use std::{error::Error, thread, time::Duration};

// Uinput is used for send the inputs
use uinput::event::controller::Mouse;

// Evdev is used for detecting keyboard input globally
use evdev::KeyCode;

pub struct Server {
    listenor: evdev::Device,
    clicker: uinput::device::Device,
    hotkey: KeyCode,
    interval: Duration,
}

impl Server {
    // Creates the server and clicker
    pub fn new(
        listenor: evdev::Device,
        interval: Duration,
        hotkey: KeyCode,
    ) -> Result<Self, Box<dyn Error>> {
        let clicker = uinput::default()?
            .name("device")?
            .event(Mouse::Left)?
            .create()?;

        Ok(Self {
            listenor,
            clicker,
            hotkey,
            interval,
        })
    }

    /// Runs the server loop
    pub fn run(&mut self) {
        println!("Server ready");
        let mut lastkeys = Vec::new();
        let mut active = false;
        loop {
            let keypresses = self
                .listenor
                .get_key_state()
                .unwrap()
                .iter()
                .collect::<Vec<KeyCode>>();

            // Check for specific key presses
            for k in &keypresses {
                if *k == self.hotkey && !lastkeys.contains(k) {
                    active = !active;
                    if active {
                        println!("Enabled clicker");
                    } else {
                        println!("Disabled clicker");
                    }
                }
            }

            if active {
                let _ = self.click();
            }

            lastkeys = keypresses;
            thread::sleep(self.interval);
        }
    }

    // Sends a left click
    pub fn click(&mut self) -> Result<(), Box<dyn Error>> {
        self.clicker.send(Mouse::Left, 1)?;
        self.clicker.send(Mouse::Left, 0)?;
        self.clicker.synchronize()?;
        Ok(())
    }
}
