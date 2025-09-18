use std::env;
use std::process::Command;

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ScrolledWindow,
    Separator, TextView,
};
use gtk4 as gtk;

const APP_ID: &str = "com.github.wayclicker";

pub fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    // otherwise this will error because of the cli
    let no_args: [&str; 0] = [];
    app.run_with_args(&no_args);
}

fn build_ui(app: &Application) {
    // Main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("wayclicker")
        .default_width(400)
        .default_height(600)
        .build();

    window.set_resizable(false);

    // Reusable
    let separator = Separator::new(Orientation::Horizontal);

    // Main container
    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    // Server control section
    let control_box = Box::new(Orientation::Horizontal, 10);
    control_box.set_margin_top(10);
    control_box.set_margin_bottom(10);

    let start_button = Button::with_label("Start Server");
    start_button.add_css_class("suggested-action");

    let stop_button = Button::with_label("Stop Server");
    stop_button.add_css_class("destructive-action");
    stop_button.set_sensitive(false);

    control_box.append(&start_button);
    control_box.append(&stop_button);
    main_box.append(&control_box);

    // Configuration section
    let config_label = Label::new(Some("Server Configuration"));
    config_label.add_css_class("heading");
    config_label.set_halign(gtk::Align::Start);
    main_box.append(&config_label);

    // Device path
    let device_box = Box::new(Orientation::Horizontal, 10);
    let device_label = Label::new(Some("Device Path:"));
    device_label.set_size_request(120, -1);
    device_label.set_halign(gtk::Align::Start);
    let device_entry = Entry::new();
    device_entry.set_placeholder_text(Some("/dev/input/event0"));
    device_box.append(&device_label);
    device_box.append(&device_entry);
    main_box.append(&device_box);

    // Interval
    let interval_box = Box::new(Orientation::Horizontal, 10);
    let interval_label = Label::new(Some("Interval (ms):"));
    interval_label.set_size_request(120, -1);
    interval_label.set_halign(gtk::Align::Start);
    let interval_entry = Entry::new();
    interval_entry.set_text("50");
    interval_box.append(&interval_label);
    interval_box.append(&interval_entry);
    main_box.append(&interval_box);

    // Keybind
    let keybind_box = Box::new(Orientation::Horizontal, 10);
    let keybind_label = Label::new(Some("Keybind:"));
    keybind_label.set_size_request(120, -1);
    keybind_label.set_halign(gtk::Align::Start);
    let keybind_entry = Entry::new();
    keybind_entry.set_text("KEY_F5");
    keybind_entry.set_placeholder_text(Some("KEY_F8"));
    keybind_box.append(&keybind_label);
    keybind_box.append(&keybind_entry);
    main_box.append(&keybind_box);

    // List devices button
    let list_devices_button = Button::with_label("List Input Devices");
    main_box.append(&separator);
    main_box.append(&list_devices_button);

    // List inputs
    let scrolled_window = ScrolledWindow::builder()
        .height_request(200)
        .vexpand(true)
        .build();

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_monospace(true);

    scrolled_window.set_child(Some(&text_view));
    scrolled_window.add_css_class("frame");
    main_box.append(&scrolled_window);

    window.set_child(Some(&main_box));

    // Basic event handlers (placeholders)
    start_button.connect_clicked(move |_| {
        println!("Start button clicked");
        let current_bin = env::current_exe().expect("Failed to get current executable path");
        let args_vec = vec![
            "server".to_string(),
            "--device".to_string(),
            device_entry.text().to_string(),
            "--interval".to_string(),
            interval_entry.text().to_string(),
            "--keybind".to_string(),
            keybind_entry.text().to_string(),
        ];

        let _cmd = Command::new("pkexec")
            .arg(current_bin)
            .args(&args_vec)
            .env("SHELL", "/bin/sh")
            .spawn();
    });

    stop_button.connect_clicked(|_| {
        println!("Stop button clicked");
    });

    list_devices_button.connect_clicked(|_| {
        println!("List devices button clicked");
    });

    window.present();
}
