use std::{error::Error, thread, time::Duration};

// Uinput is used for send the inputs
use uinput::event::controller::Mouse;

// Evdev is used for detecting keyboard input globally
use evdev::KeyCode;

struct HotKey {
    listenor: evdev::Device,
    keybind: KeyCode,
    lastkeys: Vec<KeyCode>,
    active: bool,
}

impl HotKey {
    fn new(listenor: evdev::Device, keybind: KeyCode) -> Self {
        Self {
            listenor,
            keybind,
            lastkeys: Vec::new(),
            active: false,
        }
    }

    pub fn is_active(&mut self) -> bool {
        let keypresses = self
            .listenor
            .get_key_state()
            .unwrap()
            .iter()
            .collect::<Vec<KeyCode>>();

        // Check for specific key presses
        for k in &keypresses {
            if *k == self.keybind && !self.lastkeys.contains(k) {
                self.active = !self.active;
                if self.active {
                    println!("Enabled clicker");
                } else {
                    println!("Disabled clicker");
                }
            }
        }

        self.lastkeys = keypresses;
        self.active
    }
}

pub struct Server {
    clicker: uinput::device::Device,
    hotkey: HotKey,
    interval: Duration,
}

impl Server {
    // Creates the server and clicker
    pub fn new(
        listenor: evdev::Device,
        interval: Duration,
        keybind: KeyCode,
    ) -> Result<Self, Box<dyn Error>> {
        let clicker = uinput::default()?
            .name("device")?
            .event(Mouse::Left)?
            .create()?;

        let hotkey = HotKey::new(listenor, keybind);
        Ok(Self {
            clicker,
            hotkey,
            interval,
        })
    }

    /// Runs the server loop
    pub fn run(&mut self) {
        println!("Server ready");
        loop {
            let active = self.hotkey.is_active();

            if active {
                let _ = self.click();
            }

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
