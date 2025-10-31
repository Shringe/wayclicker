use serde::Deserialize;
use std::path::PathBuf;
use std::process::exit;
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

/// Holds the parsable values behind the json packets send through the unix socket to configure the
/// server
#[derive(Deserialize, Debug)]
struct ServerPacket {
    enabled: bool,
}

impl ServerPacket {
    async fn from_packet(stream: &mut UnixStream) -> Result<Self, Box<dyn Error>> {
        let mut buffer = [0u8; 128];
        let n = stream.read(&mut buffer).await?;
        let packet = serde_json::from_slice(&buffer[..n])?;
        Ok(packet)
    }
}

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

#[allow(dead_code)]
impl ServerState {
    async fn update_with_packet(&self, packet: ServerPacket) {
        *self.enabled.write().await = packet.enabled;
    }

    /// Handles a single connection packet to the control socket
    async fn handle_connection_packet(&self, mut stream: UnixStream) -> Result<(), Box<dyn Error>> {
        let packet = ServerPacket::from_packet(&mut stream).await?;
        log::debug!("Received json packet: {:?}", packet);
        self.update_with_packet(packet).await;
        Ok(())
    }

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
    socket: PathBuf,
}

impl Server {
    /// Creates the server and clicker
    pub fn new(
        listener: evdev::Device,
        interval: Duration,
        modifiers: String,
        keybind: KeyCode,
        socket: PathBuf,
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
            socket,
        })
    }

    /// Sends a left click
    async fn click(&mut self) -> Result<(), Box<dyn Error>> {
        log::debug!("Sending a left click");
        self.clicker.send(Mouse::Left, 1)?;
        self.clicker.send(Mouse::Left, 0)?;
        self.clicker.synchronize()?;
        Ok(())
    }

    /// Runs the Unix socket listener to control the enabled state
    pub async fn listen_control_socket(&self) -> Result<(), Box<dyn Error>> {
        let listener = UnixListener::bind(&self.socket)?;
        let state = self.state.clone();

        log::info!("Control socket listening at {:?}", self.socket);
        tokio::spawn(async move {
            loop {
                let (stream, _) = match listener.accept().await {
                    Ok(conn) => conn,
                    Err(e) => {
                        log::error!("Failed to accept connection: {}", e);
                        continue;
                    }
                };

                if let Err(e) = state.handle_connection_packet(stream).await {
                    log::error!("Connection handler error: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Waits for the shutdown signal and quits
    pub async fn wait_for_shutdown(&self) {
        let socket = self.socket.clone();
        log::info!("Waiting for shutdown signal");
        tokio::spawn(async {
            match tokio::signal::ctrl_c().await {
                Ok(_) => log::debug!("Successfully recieved shutdown signal"),
                Err(e) => log::error!("Error while waiting for shutdown signal: {}", e),
            }
            log::info!("Received shutdown signal, exiting");

            std::fs::remove_file(socket).expect("Failed to clean up socket path");
            exit(0);
        });
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
                    if let Err(e) = self.click().await {
                        log::error!("Encountered an error while trying to click: {}", e);
                    }
                }
            }
        }
    }
}
