use std::{error::Error, process::exit, thread, time::Duration};

// Uinput is used for send the inputs
use uinput::event::controller::Mouse;

// Evdev is used for detecting keyboard input globally
use evdev::Device;

// Object oriented because it's easier for managing the device variable.
pub struct Server {
    device: uinput::device::Device,
    /// Amount of time to sleep in between clicks
    interval: Duration,
    active: bool,
}

impl Server {
    // Creates the server and clicker
    pub fn default() -> Result<Self, Box<dyn Error>> {
        let device = uinput::default()?
            .name("device")?
            .event(Mouse::Left)?
            .create()?;

        Ok(Self {
            device,
            interval: Duration::from_millis(50),
            active: false,
        })
    }

    /// Runs the server loop
    pub fn run(&mut self) {
        todo!();
        loop {
            if self.active {
                self.click();
            }

            thread::sleep(self.interval);
        }
    }

    // Sends a left click
    pub fn click(&mut self) -> Result<(), Box<dyn Error>> {
        self.device.send(Mouse::Left, 1)?;
        self.device.synchronize()?;
        self.device.send(Mouse::Left, 0)?;
        self.device.synchronize()?;
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
