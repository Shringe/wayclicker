use std::sync::Arc;
use std::{error::Error, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::RwLock;
use tokio::time;

// Uinput is used for send the inputs
use uinput::event::controller::Mouse;

// Evdev is used for detecting keyboard input globally
use evdev::KeyCode;

use crate::command::Command;
use crate::hotkey::HotKey;

/// Holds the rwlocks to information mutable through unix socket commands
#[derive(Clone)]
struct ServerState {
    /// Whether or not the daemon listens for hotkeys and clicks
    enabled: Arc<RwLock<bool>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            enabled: Arc::new(RwLock::new(true)),
        }
    }
}

impl ServerState {
    /// Handles a single connection to the control socket
    async fn handle_connection(&self, mut stream: UnixStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer).await?;

        let packet = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
        log::debug!("Received packet: {}", packet);

        let command = Command::from_packet(packet)?;
        let response = self.process_command(command).await;

        if let Some(response) = response {
            stream.write_all(response.as_bytes()).await?;
        }

        Ok(())
    }

    /// Processes a command and returns an optional response
    async fn process_command(&self, command: Command) -> Option<String> {
        match command {
            Command::IsEnabled => Some(self.enabled.read().await.to_string()),
            Command::SetEnabled { value } => {
                *self.enabled.write().await = value;
                None
            }
        }
    }
}

pub struct Server {
    clicker: uinput::device::Device,
    hotkey: HotKey,
    interval: Duration,
    state: ServerState,
}

impl Server {
    /// Creates the server and clicker
    pub fn new(
        listener: evdev::Device,
        interval: Duration,
        modifiers: String,
        keybind: KeyCode,
    ) -> Result<Self, Box<dyn Error>> {
        let clicker = uinput::default()?
            .name("device")?
            .event(Mouse::Left)?
            .create()?;

        let hotkey = HotKey::new(listener, modifiers, keybind);
        Ok(Self {
            clicker,
            hotkey,
            interval,
            state: ServerState::default(),
        })
    }

    /// Sends a left click
    fn click(&mut self) -> Result<(), Box<dyn Error>> {
        log::debug!("Sending a left click");
        self.clicker.send(Mouse::Left, 1)?;
        self.clicker.send(Mouse::Left, 0)?;
        self.clicker.synchronize()?;
        Ok(())
    }

    /// Runs the Unix socket listener to control the enabled state
    pub async fn listen_control_socket(
        &self,
        socket_path: &'static str,
    ) -> Result<(), Box<dyn Error>> {
        // Remove old socket if it exists
        let _ = std::fs::remove_file(socket_path);

        let listener = UnixListener::bind(socket_path)?;
        let state = self.state.clone();

        tokio::spawn(async move {
            log::info!("Control socket listening at {}", socket_path);

            loop {
                let (stream, _) = match listener.accept().await {
                    Ok(conn) => conn,
                    Err(e) => {
                        log::error!("Failed to accept connection: {}", e);
                        continue;
                    }
                };

                if let Err(e) = state.handle_connection(stream).await {
                    log::error!("Connection handler error: {}", e);
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

            if *self.state.enabled.read().await {
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
}
