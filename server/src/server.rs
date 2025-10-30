use std::sync::Arc;
use std::{error::Error, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use tokio::sync::RwLock;
use tokio::time;

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
    enabled: Arc<RwLock<bool>>,
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
            enabled: Arc::new(RwLock::new(true)),
        })
    }

    /// Spawns a Unix socket listener to control the enabled state
    pub async fn listen_control_socket(
        &self,
        socket_path: &'static str,
    ) -> Result<(), Box<dyn Error>> {
        // Remove old socket if it exists
        let _ = std::fs::remove_file(socket_path);

        let listener = UnixListener::bind(socket_path)?;
        let enabled = self.enabled.clone();

        tokio::spawn(async move {
            log::info!("Control socket listening at {}", socket_path);

            loop {
                match listener.accept().await {
                    Ok((mut stream, _)) => {
                        let enabled = enabled.clone();
                        tokio::spawn(async move {
                            let mut buffer = [0u8; 1024];

                            match stream.read(&mut buffer).await {
                                Ok(n) if n > 0 => {
                                    let command =
                                        String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                                    log::debug!("Received command: {}", command);

                                    let response = match command.as_str() {
                                        "enable" => {
                                            *enabled.write().await = true;
                                            "Enabled\n"
                                        }
                                        "disable" => {
                                            *enabled.write().await = false;
                                            "Disabled\n"
                                        }
                                        "toggle" => {
                                            let mut e = enabled.write().await;
                                            *e = !*e;
                                            if *e { "Enabled\n" } else { "Disabled\n" }
                                        }
                                        "status" => {
                                            if *enabled.read().await {
                                                "Enabled\n"
                                            } else {
                                                "Disabled\n"
                                            }
                                        }
                                        _ => {
                                            "Unknown command. Use: enable, disable, toggle, or status\n"
                                        }
                                    };

                                    let _ = stream.write_all(response.as_bytes()).await;
                                }
                                Ok(_) => log::debug!("Empty read from socket"),
                                Err(e) => log::error!("Failed to read from socket: {}", e),
                            }
                        });
                    }
                    Err(e) => log::error!("Failed to accept connection: {}", e),
                }
            }
        });

        Ok(())
    }

    /// Runs the server loop
    pub async fn run(&mut self) {
        log::info!("Server ready");
        let mut interval = time::interval(self.interval);

        loop {
            interval.tick().await;

            if *self.enabled.read().await {
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
