use serde::{Deserialize, Serialize};
use slint::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    enabled: bool,
    interval_ms: usize,
    keycode: u16,
}

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_save_clicked(move || {
        let ui = ui_handle.unwrap();

        let config = Config {
            enabled: ui.get_enabled(),
            interval_ms: ui.get_interval_ms().parse().unwrap_or(0),
            keycode: ui.get_keycode().parse().unwrap_or(0),
        };

        match serde_json::to_string_pretty(&config) {
            Ok(json) => {
                println!("Saved JSON:\n{}", json);
                std::fs::write("config.json", json).expect("Unable to write file");
            }
            Err(e) => eprintln!("Error serializing: {}", e),
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_load_clicked(move || {
        let ui = ui_handle.unwrap();

        match std::fs::read_to_string("config.json") {
            Ok(contents) => match serde_json::from_str::<Config>(&contents) {
                Ok(config) => {
                    ui.set_enabled(config.enabled);
                    ui.set_interval_ms(config.interval_ms.to_string().into());
                    ui.set_keycode(config.keycode.to_string().into());
                    println!("Loaded configuration successfully");
                }
                Err(e) => eprintln!("Error parsing JSON: {}", e),
            },
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });

    ui.run()
}
