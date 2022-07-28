mod utils;

use std::process::exit;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry, Box, Orientation, ListBox, ScrolledWindow};
use crate::utils::types::Config;
use crate::get_apps::get;
use crate::app::utils::*;

pub fn handler(app: &Application, shell: bool, config: Config) {
    app.connect_activate(move | app | {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(config.window_width)
            .resizable(false)
            .has_focus(true)
            .window_position(gtk::WindowPosition::Center)
            .decorated(false)
            .build();
        
        let container = Box::new(Orientation::Vertical, 0);

        let textbox = Entry::new();

        if shell {
            textbox.connect_key_press_event(| textbox, event | {
                if event.keycode() == Some(36) {
                    let raw_args = &textbox.text();

                    let args = raw_args
                        .split(" ")
                        .collect::<Vec<&str>>();

                    run(args);
                }

                Inhibit(false)
            });
        }

        let list = ListBox::new();

        for app in &get("/usr/share/applications") {
            add(app, &list);
        }
        
        list.connect_key_press_event(| list, event | {
            if event.keycode() == Some(36) {
                select(list, &get("/usr/share/applications"));

            }

            Inhibit(false)
        });

        list.connect_button_press_event(| list, event| {
            if event.button() == 1 {
                list.connect_row_selected(| list, _ | {
                    select(list, &get("/usr/share/applications"));
                });
            }

            Inhibit(false)
        });

        let scrollable = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .min_content_height(config.box_height)
            .child(&list)
            .build();

        textbox.connect_changed(move | text | {
            list.foreach(| i | list.remove(i));

            get("/usr/share/applications")
                .into_iter()
                .filter(| i | i.name.to_lowercase().contains(&format!("{}", text.text().to_lowercase())))
                .for_each(| i | add(&i, &list));

            list.show_all();
        });

        container.add(&textbox);
        container.add(&scrollable);
        
        window.add(&container);

        window.connect_focus_out_event(| window, _ | {
            if !window.has_focus() {
                exit(0);

            }

            Inhibit(false)
        });

        window.connect_key_press_event(move | _, event | {
            if event.keycode() == Some(9) {
                exit(0);
            
            } else if event.keycode() == Some(23) {
                scrollable.grab_focus();

            }

            Inhibit(false)
        });

        window.show_all();
    });
}
