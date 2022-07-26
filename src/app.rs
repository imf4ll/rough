use std::process::{Command, exit};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry};

pub fn handler(app: &Application) {
    app.connect_activate(| app | {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(600)
            .default_height(40)
            .title("Rough App Launcher")
            .resizable(false)
            .has_focus(true)
            .build();

        let textbox = Entry::new();

        textbox.set_placeholder_text(Some("Type your command"));

        textbox.connect_key_press_event(| textbox, event | {
            if event.keycode() == Some(36) {
                let raw_args = &textbox.text();

                let mut args = raw_args
                    .split(" ")
                    .collect::<Vec<&str>>();

                Command::new(args.swap_remove(0))
                    .args(args)
                    .spawn()
                    .expect("An error was occurred, quitting...");

                exit(0);
            
            } else if event.keycode() == Some(9) {
                exit(0);

            }

            Inhibit(false)
        });

        window.add(&textbox);

        window.connect_focus_out_event(| window, _ | {
            if !window.has_focus() {
                exit(0);

            }

            Inhibit(false)
        });

        window.show_all();
    });
}
