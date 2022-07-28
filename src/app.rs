use std::process::{Command, exit};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry, Box, Orientation, ListBox, Label, ScrolledWindow};
use crate::utils::{types::App, get_apps::get};

pub fn handler(app: &Application, shell: bool) {
    app.connect_activate(move | app | {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(700)
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

        list.connect_row_selected(| select, _ | {
            let label: String = select
                .selected_row()
                .expect("Failed to get row")
                .child()
                .expect("Failed to get child")
                .property_value("label")
                .get()
                .unwrap();

            let app = &get("/usr/share/applications")
                .into_iter()
                .filter(| i | i.name == label.split(" ").collect::<Vec<&str>>()[0])
                .collect::<Vec<App>>()[0];

            let args = app
                .exec
                .split(" ")
                .collect::<Vec<&str>>();
            
            run(args);
        });

        let scrollable = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .min_content_height(240)
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

        window.connect_key_press_event(| _, event | {
            if event.keycode() == Some(9) {
                exit(0);
            }

            Inhibit(false)
        });

        window.show_all();
    });
}

fn run(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .expect("Failed to run command");

    exit(0);
}

fn add(app: &App, list: &gtk::ListBox) {
    if app.generic != "" {
        list.add(&Label::new(Some(&format!("{} ({})", &app.name, &app.generic))));

    } else {
        list.add(&Label::new(Some(&app.name)));

    }
}
