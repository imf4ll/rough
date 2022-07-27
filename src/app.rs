use std::process::{Command, exit};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry, Box, Orientation, ListBox, Label, ScrolledWindow};
use crate::utils::get_apps::get;

pub fn handler(app: &Application) {
    app.connect_activate(| app | {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(600)
            .title("Rough")
            .resizable(false)
            .has_focus(true)
            .build();

        let container = Box::new(Orientation::Vertical, 0);

        let textbox = Entry::new();

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
            }

            Inhibit(false)
        });

        let list = ListBox::new();
        let apps = get("/usr/share/applications");

        for app in &apps {
            if app.generic != "" {
                list.add(&Label::new(Some(&format!("{} ({})", &app.name, &app.generic))));

            } else {
                list.add(&Label::new(Some(&app.name)));

            }
        }

        list.connect_row_selected(move | select, _ | {
            let app = &apps[
                select
                    .selected_row()
                    .expect("Failed to select row")
                    .index() as usize
            ];

            let mut args = app
                .exec
                .split(" ")
                .collect::<Vec<&str>>();

            Command::new(args.swap_remove(0))
                .args(args)
                .spawn()
                .expect("Failed to run command");

            exit(0);
        });

        let scrollable = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .min_content_height(250)
            .child(&list)
            .build();

        container.add(&textbox);
        container.add(&scrollable);
    
        unsafe {
            textbox.connect_changed(move | text | {
                &list.destroy();

                let value = text.text();

                &list.show_all();
            });
        }

        window.add(&container);

        window.connect_focus_out_event(| window, _ | {
            if !window.has_focus() {
                exit(0);

            }

            Inhibit(false)
        });

        window.connect_key_press_event(| _, event | {
            if event.keycode() == Some(9) {
                exit(0);
            }

            Inhibit(false)
        });

        window.show_all();
    });
}
