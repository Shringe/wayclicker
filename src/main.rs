use std::{thread, time, process::exit};

// Uinput is used for send the inputs
use uinput::event::{
    controller::Mouse::{Left, Right},
    keyboard::Key,
    keyboard::Keyboard::All,
};

// Evdev is used for detecting keyboard input globally
use evdev::{Device, KeyCode};

// Object oriented because it's easier for managing the device variable.
struct Clicker {
    device: uinput::device::Device,
}
impl Clicker {
    // Creates clicker.
    fn new() -> Clicker {
        let device = uinput::default().unwrap()
            .name("device").unwrap()
            .event(Left).unwrap()
            .event(All).unwrap()
            .create().unwrap();

        Clicker {
            device
        }
    }
    // Sends click.
    fn click(&mut self, click_type: u8) {
        match click_type {
            0 => {
                // Left Click.
                self.device.send(Left, 1).unwrap();
                self.device.synchronize().unwrap(); 
                self.device.send(Left, 0).unwrap();
                self.device.synchronize().unwrap();
            }
            1 => {
                // Right Click.
                self.device.send(Right, 1).unwrap();
                self.device.synchronize().unwrap(); 
                self.device.send(Right, 0).unwrap();
                self.device.synchronize().unwrap();
            }
            2 => {
                // Space Bar.
                self.device.click(&Key::Space).unwrap();
                self.device.synchronize().unwrap();
            }
            _ => { /* Only here so that the compiler doesn't get mad at me. */ }
        }
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

fn main() {
    let mut autoclicker = false;              // Autoclicker toggle variable.
    let mut clicker = Clicker::new();         // Clicker object.
    let mut evdevdevice = get_evdev_device(); // Evdev device.
    
    // Last frame's pressed keys.
    let mut lastkeys = vec![];

    loop {
        // Get all currently pressed keys.
        let keypresses = evdevdevice.get_key_state().unwrap().iter().collect::<Vec<KeyCode>>();
        
        // Check for specific key presses
        for k in &keypresses {
            if k.0 == 62 && !lastkeys.contains(k) {
                autoclicker = !autoclicker;
            }
        }
        
        // Update lastkeys.
        lastkeys = keypresses;

        // Repeatedly click when the autoclicker is on.
        if autoclicker {
            clicker.click(0);
            thread::sleep(time::Duration::from_millis(50));
        }
    }
}
