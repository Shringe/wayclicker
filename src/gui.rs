use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::process::{Child, Command};
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ScrolledWindow,
    Separator, TextView,
};
use gtk4::{self as gtk, DropDown, StringList, gdk};

const APP_ID: &str = "com.github.wayclicker";

pub fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    let client = Rc::new(RefCell::new(Client::default()));
    app.connect_activate(move |app| build_ui(app, client.clone()));

    // otherwise this will error because of the cli
    let no_args: [&str; 0] = [];
    app.run_with_args(&no_args);
}

/// Stores the state of the GUI
#[derive(Default)]
struct Client {
    server_process: Option<Child>,
    /// Maps device names to paths
    device_map: HashMap<String, String>,
    capture_keybind: bool,
}

fn build_ui(app: &Application, client: Rc<RefCell<Client>>) {
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
    let binding = client.clone();
    let client_for_main = binding.borrow();

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

    let is_server_running = client_for_main.server_process.is_some();
    let start_button = Button::with_label("Start Server");
    start_button.add_css_class("suggested-action");
    start_button.set_sensitive(!is_server_running);

    let stop_button = Button::with_label("Stop Server");
    stop_button.add_css_class("destructive-action");
    stop_button.set_sensitive(is_server_running);

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

    let device_names: Vec<String> = client_for_main.device_map.keys().cloned().collect();
    let names_str: Vec<&str> = device_names.iter().map(|s| s.as_str()).collect();
    let string_list = StringList::new(&names_str);
    let device_menu = DropDown::new(Some(string_list), None::<gtk::Expression>);
    // device_menu.set_placeholder_text(Some("/dev/input/event0"));

    device_box.append(&device_label);
    device_box.append(&device_menu);
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
    let key_controller = gtk::EventControllerKey::new();
    let keybind_entry_capture_label = "Click to set hotkey...";
    let keybind_entry = Button::with_label(keybind_entry_capture_label);
    keybind_label.set_size_request(120, -1);
    keybind_label.set_halign(gtk::Align::Start);
    keybind_entry.set_focus_on_click(true);
    keybind_entry.add_controller(key_controller.clone());
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
    let text_buffer = text_view.buffer();
    text_view.set_editable(false);
    text_view.set_monospace(true);

    scrolled_window.set_child(Some(&text_view));
    scrolled_window.add_css_class("frame");
    main_box.append(&scrolled_window);

    window.set_child(Some(&main_box));

    // Event handlers
    let client_for_start = client.clone();
    let device_menu_for_start = device_menu.clone();
    let start_for_start = start_button.clone();
    let stop_for_start = stop_button.clone();
    let keybind_entry_for_start = keybind_entry.clone();
    start_button.connect_clicked(move |_| {
        println!("Start button clicked");
        let mut client = client_for_start.borrow_mut();
        if let Err(e) = start_server(
            &mut client,
            &device_menu_for_start,
            &start_for_start,
            &stop_for_start,
            &keybind_entry_for_start,
            &interval_entry,
        ) {
            eprintln!("Failed to start server: {}", e);
        };
    });

    let client_for_stop = client.clone();
    let start_for_stop = start_button.clone();
    let stop_for_stop = stop_button.clone();
    stop_button.connect_clicked(move |_| {
        println!("Stop button clicked");
        let mut client = client_for_stop.borrow_mut();
        let start = &start_for_stop;
        let stop = &stop_for_stop;
        if let Some(child) = client.server_process.take() {
            let _result = Command::new("pkexec")
                .arg("kill")
                .arg(child.id().to_string())
                .env("SHELL", "/bin/sh")
                .spawn();

            start.set_sensitive(true);
            stop.set_sensitive(false);
        }
    });

    let client_for_list = client.clone();
    let device_menu_for_list = device_menu.clone();
    list_devices_button.connect_clicked(move |_| {
        println!("List devices button clicked");
        let mut client = client_for_list.borrow_mut();
        let device_menu = &device_menu_for_list;

        let current_bin = env::current_exe().expect("Failed to get current executable path");
        let args_vec = vec!["list".to_string()];
        let result = Command::new("pkexec")
            .arg(current_bin)
            .args(&args_vec)
            .env("SHELL", "/bin/sh")
            .output()
            .expect("Couldn't list devices");

        let device_list =
            String::from_utf8(result.stdout).expect("Couldn't parse device list stdout");

        text_buffer.set_text(&device_list);

        let mut device_map = HashMap::new();
        for line in device_list.lines() {
            if let Some(colon_pos) = line.find(':') {
                let path_part = &line[..colon_pos];
                let name_part = &line[colon_pos + 1..];

                let device_path = path_part.trim_matches('"');
                let device_name = name_part.trim();

                device_map.insert(device_name.to_string(), device_path.to_string());
            }
        }

        client.device_map = device_map;

        let device_names: Vec<String> = client.device_map.keys().cloned().collect();
        let names_str: Vec<&str> = device_names.iter().map(|s| s.as_str()).collect();
        let string_list = StringList::new(&names_str);

        device_menu.set_model(Some(&string_list));
    });

    let client_for_keybind = client.clone();
    let keybind_entry_for_keybind = keybind_entry.clone();
    keybind_entry.connect_clicked(move |_| {
        println!("Keybind button clicked");
        let mut client = client_for_keybind.borrow_mut();
        let keybind_entry = &keybind_entry_for_keybind;

        if !client.capture_keybind {
            client.capture_keybind = true;
            keybind_entry.set_label(keybind_entry_capture_label);
        }
    });

    let client_for_controller = client.clone();
    let keybind_entry_for_controller = keybind_entry.clone();
    key_controller.connect_key_pressed(move |controller, keyval, _keycode, modifiers| {
        let mut client = client_for_controller.borrow_mut();
        let keybind_entry = &keybind_entry_for_controller;
        if !client.capture_keybind || gtk_key_is_modifier(&keyval) {
            return gtk::glib::Propagation::Proceed;
        }

        if let Some(widget) = controller.widget() {
            if widget == keybind_entry.clone().upcast::<gtk::Widget>() {
                if let Some(hotkey) = gtk_to_evdev_keyname(&keyval) {
                    println!("{:#?}", modifiers);
                    let mut label = String::new();
                    for m in modifiers.iter_names() {
                        let name = if let Some(p) = gtk_modifier_to_pretty(m.0) {
                            p
                        } else {
                            m.0
                        };

                        label.push_str((name.to_string() + "+").as_str());
                    }

                    label.push_str(hotkey.as_str());
                    keybind_entry.set_label(label.as_str());

                    client.capture_keybind = false;
                    return gtk::glib::Propagation::Stop;
                }
            }
        }
        gtk::glib::Propagation::Proceed
    });

    window.present();
}

/// Translates gdk keyvalues to evdev keynames as best as possible
fn gtk_to_evdev_keyname(keyval: &gdk::Key) -> Option<String> {
    match *keyval {
        gdk::Key::Control_L => Some("KEY_LEFTCTRL".to_string()),
        gdk::Key::Control_R => Some("KEY_RIGHTCTRL".to_string()),
        gdk::Key::Shift_L => Some("KEY_LEFTSHIFT".to_string()),
        gdk::Key::Shift_R => Some("KEY_RIGHTSHIFT".to_string()),
        gdk::Key::Super_L => Some("KEY_LEFTMETA".to_string()),
        gdk::Key::Super_R => Some("KEY_RIGHTMETA".to_string()),
        gdk::Key::Alt_L => Some("KEY_LEFTALT".to_string()),
        gdk::Key::Alt_R => Some("KEY_RIGHTALT".to_string()),
        _ => {
            let name = "key_".to_string() + keyval.name()?.as_str();
            Some(name.to_uppercase())
        }
    }
}

/// Maps gtk modifier names to evdev names using the server's "-" syntax for left/right alternatives.
fn gtk_modifier_name_to_evdev(modifier: &str) -> Option<&str> {
    match modifier {
        "ALT_MASK" => Some("KEY_LEFTALT-KEY_RIGHTALT"),
        "SUPER_MASK" => Some("KEY_LEFTMETA-KEY_RIGHTMETA"),
        "SHIFT_MASK" => Some("KEY_LEFTSHIFT-KEY_RIGHTSHIFT"),
        "CONTROL_MASK" => Some("KEY_LEFTCTRL-KEY_RIGHTCTRL"),
        _ => None,
    }
}

fn gtk_modifier_to_pretty(modifier: &str) -> Option<&str> {
    match modifier {
        "ALT_MASK" => Some("Alt"),
        "SUPER_MASK" => Some("Super"),
        "SHIFT_MASK" => Some("Shift"),
        "CONTROL_MASK" => Some("Control"),
        _ => None,
    }
}

fn gtk_modifier_from_pretty(modifier: &str) -> Option<&str> {
    match modifier {
        "Alt" => Some("ALT_MASK"),
        "Super" => Some("SUPER_MASK"),
        "Shift" => Some("SHIFT_MASK"),
        "Control" => Some("CONTROL_MASK"),
        _ => None,
    }
}

fn gtk_key_is_modifier(keyval: &gdk::Key) -> bool {
    match *keyval {
        gdk::Key::Control_L
        | gdk::Key::Control_R
        | gdk::Key::Shift_L
        | gdk::Key::Shift_R
        | gdk::Key::Super_L
        | gdk::Key::Super_R
        | gdk::Key::Alt_L
        | gdk::Key::Alt_R => true,
        _ => false,
    }
}

fn start_server(
    client: &mut Client,
    device_menu: &gtk::DropDown,
    start: &gtk::Button,
    stop: &gtk::Button,
    keybind_entry: &gtk::Button,
    interval_entry: &gtk::Entry,
) -> anyhow::Result<()> {
    let current_bin = env::current_exe()?;

    let device_item = device_menu
        .selected_item()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get dropdown entry"))?;
    let device_object = device_item
        .downcast::<gtk::StringObject>()
        .map_err(|_| anyhow::anyhow!("Couldn't get dropdown object"))?;
    let device_name = device_object.string().to_string();
    let device_path = client
        .device_map
        .get(&device_name)
        .ok_or_else(|| anyhow::anyhow!("Couldn't find device path"))?;

    let mut args_vec = vec![
        "server".to_string(),
        "--device".to_string(),
        device_path.to_string(),
        "--interval".to_string(),
        interval_entry.text().to_string(),
    ];

    let keybind_label = keybind_entry
        .label()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get keybind label"))?;
    let mut keybind_parts = keybind_label.split('+').rev();
    let key = keybind_parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("No keybind found"))?;

    args_vec.push("--keybind".to_string());
    args_vec.push(key.to_string());
    args_vec.push("--modifiers".to_string());

    let modifiers: Vec<String> = keybind_parts
        .into_iter()
        .filter_map(|part| {
            let formatted_part = if let Some(p) = gtk_modifier_from_pretty(part) {
                p
            } else {
                part
            };
            gtk_modifier_name_to_evdev(formatted_part)
        })
        .map(|s| s.to_string())
        .collect();
    args_vec.push(modifiers.join("+"));

    let child = Command::new("pkexec")
        .arg(current_bin)
        .args(&args_vec)
        .env("SHELL", "/bin/sh")
        .spawn()?;

    client.server_process = Some(child);
    start.set_sensitive(false);
    stop.set_sensitive(true);

    Ok(())
}
