use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::UnixStream};

/// Holds the parsable values behind the json packets send through the unix socket to configure the
/// server
#[derive(Deserialize, Serialize, Debug)]
pub struct ServerPacket {
    pub enabled: bool,
    pub interval_ms: u64,
    pub hotkey: u16,
}

impl ServerPacket {
    pub async fn from_packet(stream: &mut UnixStream) -> Result<Self, Box<dyn Error>> {
        let mut buffer = [0u8; 128];
        let n = stream.read(&mut buffer).await?;
        let packet = serde_json::from_slice(&buffer[..n])?;
        Ok(packet)
    }
}
