// Catisen Browser - Minimal Rust/Servo + GTK4 Starter
// This is a minimal browser window with a Ctrl+K address bar.

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry, Overlay, Image, EventControllerKey};

pub fn run() {
    // Initialize GTK
    let app = Application::builder()
        .application_id("com.catisen.browser")
        .build();

    app.connect_activate(|app| {
        // Create a window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Catisen")
            .default_width(1200)
            .default_height(800)
            .build();

        // Setup Overlay
        let overlay = Overlay::new();
        
        // We can add a fallback background if the image isn't present
        let bg = Image::from_file("assets/mentality_bg.png");
        overlay.set_child(Some(&bg));

        // Address bar
        let address_bar = Entry::builder()
            .placeholder_text("Search or enter address...")
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();

        // Keyboard shortcuts
        let key_controller = EventControllerKey::new();
        let window_clone = window.clone();
        key_controller.connect_key_pressed({
            let address_bar = address_bar.clone();
            move |_, keyval, _keycode, state| {
                if state.contains(gtk::gdk::ModifierType::CONTROL_MASK) {
                    if keyval == gtk::gdk::Key::k {
                        address_bar.grab_focus();
                        return glib::Propagation::Stop;
                    } else if keyval == gtk::gdk::Key::comma {
                        let settings_win = crate::settings_ui::build_settings_window(&window_clone);
                        settings_win.present();
                        return glib::Propagation::Stop;
                    }
                }
                glib::Propagation::Proceed
            }
        });
        window.add_controller(key_controller);

        overlay.add_overlay(&address_bar);
        window.set_child(Some(&overlay));
        window.present();
    });

    app.run();
}