use file_owner::PathExt;
use lib::ServerPacket;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::{error::Error, time::Duration};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::RwLock;
use tokio::time;

// Uinput is used for send the inputs
use uinput::event::controller::Mouse;

// Evdev is used for detecting keyboard input globally
use evdev::KeyCode;

use crate::hotkey::HotKey;

/// Holds the rwlocks to information mutable through unix socket commands
#[derive(Clone)]
struct ServerState {
    /// Whether or not the daemon listens for hotkeys and clicks
    enabled: Arc<RwLock<bool>>,
    /// The duration interval between clicks
    interval: Arc<RwLock<Duration>>,

    /// The keycode for the hotkey
    hotkey: Arc<RwLock<HotKey>>,
}

impl ServerState {
    pub fn new(hotkey: HotKey) -> Self {
        Self {
            enabled: Arc::new(RwLock::new(true)),
            interval: Arc::new(RwLock::new(Duration::from_millis(50))),
            hotkey: Arc::new(RwLock::new(hotkey)),
        }
    }

    async fn update_with_packet(&self, packet: ServerPacket) {
        *self.enabled.write().await = packet.enabled;
        *self.interval.write().await = Duration::from_millis(packet.interval_ms);
        self.hotkey.write().await.keybind = KeyCode::new(packet.hotkey);
    }

    /// Handles a single connection packet to the control socket
    async fn handle_connection_packet(&self, mut stream: UnixStream) -> Result<(), Box<dyn Error>> {
        let packet = ServerPacket::from_packet(&mut stream).await?;
        log::debug!("Received json packet: {:?}", packet);
        self.update_with_packet(packet).await;
        Ok(())
    }
}

pub struct Server {
    clicker: uinput::device::Device,
    socket_path: PathBuf,
    socket_group: String,
    hotkey_poll_interval: Duration,
    state: ServerState,
}

impl Server {
    /// Creates the server and clicker
    pub fn new(
        listener: evdev::Device,
        modifiers: String,
        keybind: KeyCode,
        socket_path: PathBuf,
        socket_group: String,
        hotkey_poll_interval: Duration,
    ) -> Result<Self, Box<dyn Error>> {
        let clicker = uinput::default()?
            .name("device")?
            .event(Mouse::Left)?
            .create()?;

        let hotkey = HotKey::new(listener, modifiers, keybind);
        Ok(Self {
            clicker,
            socket_path,
            socket_group,
            hotkey_poll_interval,
            state: ServerState::new(hotkey),
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

    /// Creates the unix socket and applies permissions
    async fn create_socket(&self) -> Result<UnixListener, Box<dyn Error>> {
        log::debug!("Creating listener and setting permissions");
        let listener = UnixListener::bind(&self.socket_path)?;
        self.socket_path.set_group(self.socket_group.as_str())?;
        fs::set_permissions(&self.socket_path, fs::Permissions::from_mode(0o770))?;
        Ok(listener)
    }

    /// Runs the Unix socket listener to control the enabled state
    pub async fn listen_control_socket(&self) -> Result<(), Box<dyn Error>> {
        let listener = self.create_socket().await?;
        let state = self.state.clone();

        log::info!("Control socket listening at {:?}", self.socket_path);
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
        let socket = self.socket_path.clone();
        log::info!("Waiting for shutdown signal");
        tokio::spawn(async {
            match tokio::signal::ctrl_c().await {
                Ok(_) => log::debug!("Successfully recieved shutdown signal"),
                Err(e) => log::error!("Error while waiting for shutdown signal: {}", e),
            }
            log::info!("Received shutdown signal, exiting");

            fs::remove_file(socket).expect("Failed to clean up socket path");
            exit(0);
        });
    }

    pub async fn listen_for_hotkey(&self) {
        log::info!("Listening for hotkey");
        let hotkey = self.state.hotkey.clone();
        let mut polling_rate = time::interval(self.hotkey_poll_interval);
        tokio::spawn(async move {
            loop {
                polling_rate.tick().await;
                hotkey
                    .write()
                    .await
                    .update()
                    .await
                    .expect("Failed to determine if the hotkey is active");
            }
        });
    }

    /// Runs the server loop
    pub async fn run(&mut self) {
        log::info!("Server ready");
        let mut interval = time::interval(*self.state.interval.read().await);

        loop {
            interval.tick().await;

            if *self.state.enabled.read().await {
                let current_interval = *self.state.interval.read().await;
                if interval.period() != current_interval {
                    interval = time::interval(current_interval);
                }

                if self.state.hotkey.read().await.active {
                    if let Err(e) = self.click().await {
                        log::error!("Encountered an error while trying to click: {}", e);
                    }
                }
            }
        }
    }
}
