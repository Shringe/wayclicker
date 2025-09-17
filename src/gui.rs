use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ScrolledWindow,
    Separator, TextView,
};

const APP_ID: &str = "com.example.autoclicker";

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
        .title("Autoclicker GUI")
        .default_width(600)
        .default_height(400)
        .build();

    window.set_resizable(false);

    // Main container
    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    // Title
    let title = Label::new(Some("Autoclicker Control Panel"));
    title.add_css_class("title-1");
    main_box.append(&title);

    // Separator
    let separator1 = Separator::new(Orientation::Horizontal);
    main_box.append(&separator1);

    // Server control section
    let control_box = Box::new(Orientation::Horizontal, 10);
    control_box.set_margin_top(10);
    control_box.set_margin_bottom(10);

    let start_button = Button::with_label("Start Server");
    start_button.add_css_class("suggested-action");

    let stop_button = Button::with_label("Stop Server");
    stop_button.add_css_class("destructive-action");
    stop_button.set_sensitive(false);

    let quit_button = Button::with_label("Quit");
    quit_button.add_css_class("destructive-action");

    control_box.append(&start_button);
    control_box.append(&stop_button);
    control_box.append(&quit_button);

    main_box.append(&control_box);

    // Configuration section
    let config_label = Label::new(Some("Server Configuration"));
    config_label.add_css_class("heading");
    config_label.set_halign(gtk4::Align::Start);
    main_box.append(&config_label);

    // Device path
    let device_box = Box::new(Orientation::Horizontal, 10);
    let device_label = Label::new(Some("Device Path:"));
    device_label.set_size_request(120, -1);
    device_label.set_halign(gtk4::Align::Start);
    let device_entry = Entry::new();
    device_entry.set_placeholder_text(Some("/dev/input/event0"));
    device_box.append(&device_label);
    device_box.append(&device_entry);
    main_box.append(&device_box);

    // Interval
    let interval_box = Box::new(Orientation::Horizontal, 10);
    let interval_label = Label::new(Some("Interval (ms):"));
    interval_label.set_size_request(120, -1);
    interval_label.set_halign(gtk4::Align::Start);
    let interval_entry = Entry::new();
    interval_entry.set_text("50");
    interval_box.append(&interval_label);
    interval_box.append(&interval_entry);
    main_box.append(&interval_box);

    // Keybind
    let keybind_box = Box::new(Orientation::Horizontal, 10);
    let keybind_label = Label::new(Some("Keybind:"));
    keybind_label.set_size_request(120, -1);
    keybind_label.set_halign(gtk4::Align::Start);
    let keybind_entry = Entry::new();
    keybind_entry.set_text("KEY_F5");
    keybind_entry.set_placeholder_text(Some("KEY_F8"));
    keybind_box.append(&keybind_label);
    keybind_box.append(&keybind_entry);
    main_box.append(&keybind_box);

    // List devices button
    let list_devices_button = Button::with_label("List Input Devices");
    main_box.append(&list_devices_button);

    // Separator
    let separator2 = Separator::new(Orientation::Horizontal);
    main_box.append(&separator2);

    // Output section
    let output_label = Label::new(Some("Device List / Server Output"));
    output_label.add_css_class("heading");
    output_label.set_halign(gtk4::Align::Start);
    main_box.append(&output_label);

    let scrolled_window = ScrolledWindow::builder()
        .height_request(200)
        .vexpand(true)
        .build();

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_monospace(true);

    scrolled_window.set_child(Some(&text_view));
    main_box.append(&scrolled_window);

    window.set_child(Some(&main_box));

    // Basic event handlers (placeholders)
    start_button.connect_clicked(|_| {
        println!("Start button clicked");
    });

    stop_button.connect_clicked(|_| {
        println!("Stop button clicked");
    });

    list_devices_button.connect_clicked(|_| {
        println!("List devices button clicked");
    });

    quit_button.connect_clicked(move |_| {
        println!("Quit button clicked");
    });

    window.present();
}
