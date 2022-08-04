mod utils;

use std::process::exit;

use gtk::prelude::*;
use crate::utils::types::Config;
use crate::apps::get;
use crate::app::utils::*;
use crate::styling::provider;
use gtk::{
    Application,
    ApplicationWindow,
    Entry,
    Box,
    Orientation,
    ListBox,
    ScrolledWindow
};

pub fn build_app(app: &Application, shell: bool, config: Config) {
    app.connect_activate(move | app | {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(config.window.width)
            .resizable(false)
            .has_focus(true)
            .window_position(gtk::WindowPosition::CenterAlways)
            .decorated(false)
            .border_width(config.window.border_width as u32)
            .title("Rough")
            .build();

        if config.window.opacity < 1.0 {
            window.set_app_paintable(true);

            set_visual(&window, None);

            window.connect_screen_changed(set_visual);
            
            let config_clone = config.clone();

            window.connect_draw(move | window, ctx | {
                draw(window, ctx, &config_clone)
            });
        }

        window
            .style_context()
            .add_provider(&provider(&config), gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        
        let container = Box::new(Orientation::Vertical, 0);

        let textbox = Entry::new();

        textbox
            .style_context()
            .add_provider(&provider(&config), gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        textbox.set_placeholder_text(Some("Type your command or app"));
        textbox.set_margin(config.textbox.margin);
        textbox.set_app_paintable(true);
        textbox.set_vexpand(true);
        textbox.set_xalign(0.5);

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

        list
            .style_context()
            .add_provider(&provider(&config), gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        
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
            .margin_top(config.list.margin_top)
            .build();

        let container_clone = container.clone();
        let scrollable_clone = scrollable.clone();
        let config_clone = config.clone();

        textbox.connect_changed(move | text | {
            if !format!("{:?}", container_clone.children()).contains("GtkScrolledWindow") {
                if !format!("{:?}", scrollable_clone.children()).contains("GtkViewport") {
                    scrollable_clone.add(&list);

                }

                container_clone.add(&scrollable_clone);
                container_clone.show_all();
            }

            list.foreach(| i | list.remove(i));

            get("/usr/share/applications")
                .into_iter()
                .filter(| i |
                    i.name.to_lowercase().contains(&format!("{}", text.text().to_lowercase())) ||
                    i.exec.contains(&format!("{}", text.text().to_lowercase()))
                )
                .for_each(| i | add(&i, &list, &config_clone));

            if text.text() == "" || list.children().len() == 0 {
                container_clone.remove(&scrollable_clone);

            }

            list.show_all();

            let size = match list.children().len() as i32 {
                n if n <= 2 => 0,
                n if n <= 3 => 75,
                n if n <= 5 => 100,
                _ => config.container.max_height,
            };

            scrollable_clone.set_height_request(size);
            scrollable_clone.show_all();
        });

        container.add(&textbox);
        
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
