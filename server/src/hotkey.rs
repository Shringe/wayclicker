use evdev::{EnumParseError, KeyCode};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

pub struct HotKey {
    listener: evdev::Device,
    modifiers: String,
    keybind: KeyCode,
    lastkeys: Vec<KeyCode>,
    pub active: Arc<RwLock<bool>>,
}

impl HotKey {
    pub fn new(listener: evdev::Device, modifiers: String, keybind: KeyCode) -> Self {
        Self {
            listener,
            modifiers,
            keybind,
            lastkeys: Vec::new(),
            active: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn update(&mut self) -> Result<(), EnumParseError> {
        let keypresses = self
            .listener
            .get_key_state()
            .expect("Failed to get keypresses from listener")
            .iter()
            .collect::<Vec<KeyCode>>();

        // Make sure all modifiers are pressed
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
                        self.lastkeys = keypresses;
                        return Ok(());
                    }
                } else {
                    // Must contain this single modifier
                    let key = evdev::KeyCode::from_str(m)?;
                    if !keypresses.contains(&key) {
                        self.lastkeys = keypresses;
                        return Ok(());
                    }
                }
            }
        }

        // Check for specific key presses
        let mut active = *self.active.read().await;
        for k in &keypresses {
            if *k == self.keybind && !self.lastkeys.contains(k) {
                active = !active;
                if active {
                    log::info!("Enabled clicker");
                } else {
                    log::info!("Disabled clicker");
                }
            }
        }

        *self.active.write().await = active;
        self.lastkeys = keypresses;
        Ok(())
    }
}
