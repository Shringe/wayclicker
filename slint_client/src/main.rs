use lib::ServerPacket;
use std::{
    io::{self, Write},
    num::ParseIntError,
    os::unix::net::UnixStream,
};

const SOCKET_PATH: &str = "/tmp/wayclicker.sock";

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    let ui = MainWindow::new()?;

    let default_key_code = evdev::KeyCode::KEY_F8.code();
    ui.set_keycode(default_key_code.to_string().into());

    let ui_handle = ui.as_weak();
    ui.on_send_server_packet(move || {
        let ui = ui_handle.unwrap();
        let packet = match formulate_packet(&ui) {
            Ok(packet) => packet,
            Err(e) => {
                log::error!("Failed to formulate packet: {}", e);
                return;
            }
        };

        log::debug!("Created packet: {:?}", packet);
        let json = match serde_json::to_string(&packet) {
            Ok(json) => json,
            Err(e) => {
                log::error!("Failed to serialize packet: {}", e);
                return;
            }
        };

        match send_packet(&json) {
            Ok(_) => log::debug!("Successfully send packet to server"),
            Err(e) => log::error!("Failed to send packet to server: {}", e),
        }
    });

    ui.run()
}

fn send_packet(packet: &String) -> io::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    stream.write_all(packet.as_bytes())?;
    stream.flush()?;
    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}

fn formulate_packet(ui: &MainWindow) -> Result<ServerPacket, ParseIntError> {
    Ok(ServerPacket {
        enabled: ui.get_enabled(),
        interval_ms: ui.get_interval().parse()?,
        hotkey: ui.get_keycode().parse()?,
    })
}
