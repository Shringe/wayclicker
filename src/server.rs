use std::{error::Error, str::FromStr, thread, time::Duration};

// Uinput is used for send the inputs
use uinput::event::controller::Mouse;

// Evdev is used for detecting keyboard input globally
use evdev::KeyCode;

struct HotKey {
    listenor: evdev::Device,
    modifiers: String,
    keybind: KeyCode,
    lastkeys: Vec<KeyCode>,
    active: bool,
}

impl HotKey {
    fn new(listenor: evdev::Device, modifiers: String, keybind: KeyCode) -> Self {
        Self {
            listenor,
            modifiers,
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

        // Make sure all modifiers are pressed
        for m in self.modifiers.split('+') {
            if m.contains('-') {
                // Must contain ONE of these split modifiers
                let either_mods = m.split('-');
                let mut atleast_one_pressed = false;
                for either in either_mods {
                    let key = evdev::KeyCode::from_str(either).unwrap();
                    if keypresses.contains(&key) {
                        atleast_one_pressed = true;
                        break;
                    }
                }

                if !atleast_one_pressed {
                    self.lastkeys = keypresses;
                    return self.active;
                }
            } else {
                // Must contain this single modifier
                let key = evdev::KeyCode::from_str(m).unwrap();
                if !keypresses.contains(&key) {
                    self.lastkeys = keypresses;
                    return self.active;
                }
            }
        }

        // Check for specific key presses
        for k in &keypresses {
            if *k == self.keybind && !self.lastkeys.contains(k) {
                self.active = !self.active;
                if self.active {
                    log::info!("Enabled clicker");
                } else {
                    log::info!("Disabled clicker");
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
                let active = self.hotkey.is_active();

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
