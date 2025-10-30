use evdev::KeyCode;
use std::str::FromStr;

pub struct HotKey {
    listenor: evdev::Device,
    modifiers: String,
    keybind: KeyCode,
    lastkeys: Vec<KeyCode>,
    active: bool,
}

impl HotKey {
    pub fn new(listenor: evdev::Device, modifiers: String, keybind: KeyCode) -> Self {
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
