use evdev::{EnumParseError, KeyCode};
use std::str::FromStr;

pub struct HotKey {
    listener: evdev::Device,
    modifiers: String,
    pub keybind: KeyCode,
    lastkeys: Vec<KeyCode>,
    pub active: bool,
}

impl HotKey {
    pub fn new(listener: evdev::Device, modifiers: String, keybind: KeyCode) -> Self {
        Self {
            listener,
            modifiers,
            keybind,
            lastkeys: Vec::new(),
            // active: Arc::new(RwLock::new(false)),
            active: false,
        }
    }

    pub async fn update(&mut self) -> Result<(), EnumParseError> {
        let keypresses = self
            .listener
            .get_key_state()
            .expect("Failed to get keypresses from listener")
            .iter()
            .collect::<Vec<KeyCode>>();

        macro_rules! end {
            () => {{
                self.lastkeys = keypresses;
                return Ok(());
            }};
        }

        // Check if the keybind was just pressed (cheap check first)
        let keybind_just_pressed =
            keypresses.contains(&self.keybind) && !self.lastkeys.contains(&self.keybind);

        if !keybind_just_pressed {
            end!()
        }

        // Ensure modifiers are pressed
        if !self.modifiers.is_empty() {
            for m in self.modifiers.split('+') {
                if m.contains('-') {
                    // Must contain ONE of these split modifiers
                    let either_mods = m.split('-');
                    let mut atleast_one_pressed = false;
                    for either in either_mods {
                        let key = evdev::KeyCode::from_str(either)?;
                        if keypresses.contains(&key) {
                            atleast_one_pressed = true;
                            break;
                        }
                    }

                    if !atleast_one_pressed {
                        end!()
                    }
                } else {
                    // Must contain this single modifier
                    let key = evdev::KeyCode::from_str(m)?;
                    if !keypresses.contains(&key) {
                        end!()
                    }
                }
            }
        }

        // Toggle active state
        self.active = !self.active;
        if self.active {
            log::info!("Enabled clicker");
        } else {
            log::info!("Disabled clicker");
        }

        end!()
    }
}
