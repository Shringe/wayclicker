use std::{error::Error, process::exit};

// Uinput is used for send the inputs
use uinput::event::{
    controller::Mouse::{Left, Right},
    keyboard::Key,
    keyboard::Keyboard::All,
};

// Evdev is used for detecting keyboard input globally
use evdev::{Device, KeyCode};

// Object oriented because it's easier for managing the device variable.
pub struct Server {
    device: uinput::device::Device,
}

impl Server {
    // Creates clicker.
    pub fn new() -> Self {
        let device = uinput::default()
            .unwrap()
            .name("device")
            .unwrap()
            .event(Left)
            .unwrap()
            .event(All)
            .unwrap()
            .create()
            .unwrap();

        Self { device }
    }

    // Sends click.
    pub fn click(&mut self, click_type: u8) -> Result<(), Box<dyn Error>> {
        match click_type {
            0 => {
                // Left Click.
                self.device.send(Left, 1)?;
                self.device.synchronize()?;
                self.device.send(Left, 0)?;
                self.device.synchronize()?;
            }
            1 => {
                // Right Click.
                self.device.send(Right, 1)?;
                self.device.synchronize()?;
                self.device.send(Right, 0)?;
                self.device.synchronize()?;
            }
            2 => {
                // Space Bar.
                self.device.click(&Key::Space)?;
                self.device.synchronize()?;
            }
            _ => { /* Only here so that the compiler doesn't get mad at me. */ }
        }
        Ok(())
    }
}

// Get device for evdev.
fn get_evdev_device() -> Device {
    // Get all possible devices.
    let mut devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    devices.reverse();

    // Check for any devices related to the keyboard.
    let mut keyid = 255;
    for (i, d) in devices.iter().enumerate() {
        if d.name().unwrap_or("").to_lowercase().contains("keyboard") {
            keyid = i;
        }
    }
    // If no keyboard is found, crash.
    if keyid == 255 {
        eprintln!("No keyboard device found.");
        exit(1);
    }

    // Return the keyboard device.
    devices.into_iter().nth(keyid).unwrap()
}
