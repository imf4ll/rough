mod transparency;

use std::process::{Command, exit};
use std::fs::{read_dir, read_to_string};

use gtk::prelude::*;
use crate::utils::types::Config;
use crate::app::transparency::*;
use crate::styling::Provider;
use gtk::{
    Application,
    ApplicationWindow,
    Entry,
    Box,
    Orientation,
    ListBox,
    ScrolledWindow,
    Label,
    Image,
    IconSize,
    Align,
};

pub struct App {
    pub app: Application,
    pub shell: bool,
    pub config: Config,
}

impl App {
    pub fn run(self) {
        self.app.connect_activate(move | app | {
            let provider = Provider {
                config: self.config.clone(),
            }
                .new();

            let window = ApplicationWindow::builder()
                .application(app)
                .default_width(self.config.window.width)
                .resizable(false)
                .has_focus(true)
                .window_position(gtk::WindowPosition::CenterAlways)
                .decorated(false)
                .border_width(self.config.window.border_width as u32)
                .title("Rough")
                .build();

            if self.config.window.opacity < 1.0 {
                window.set_app_paintable(true);

                set_visual(&window, None);

                window.connect_screen_changed(set_visual);
                
                let config_clone = self.config.clone();

                window.connect_draw(move | window, ctx | {
                    draw(window, ctx, &config_clone)
                });
            }

            window
                .style_context()
                .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            
            let container = Box::new(Orientation::Vertical, 0);

            let textbox = Entry::new();

            textbox
                .style_context()
                .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

            textbox.set_placeholder_text(Some("Type your command or app"));
            textbox.set_margin(self.config.textbox.margin);
            textbox.set_app_paintable(true);
            textbox.set_vexpand(true);
            textbox.set_xalign(0.5);

            if self.shell {
                textbox.connect_key_press_event(| textbox, event | {
                    if event.keycode() == Some(36) {
                        let raw_args = &textbox.text();

                        let args = raw_args
                            .split(" ")
                            .collect::<Vec<&str>>();

                        Self::exec(args);
                    }

                    Inhibit(false)
                });
            }

            let list = ListBox::new();

            list
                .style_context()
                .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            
            list.connect_key_press_event(| list, event | {
                if event.keycode() == Some(36) {
                    Self::select(list, &Self::get_apps("/usr/share/applications"));

                }

                Inhibit(false)
            });

            list.connect_button_press_event(| list, event| {
                if event.button() == 1 {
                    list.connect_row_selected(| list, _ | {
                        Self::select(list, &Self::get_apps("/usr/share/applications"));
                    });
                }

                Inhibit(false)
            });

            let scrollable = ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .margin_top(self.config.list.margin_top)
                .build();

            let container_clone = container.clone();
            let scrollable_clone = scrollable.clone();
            let config_clone = self.config.clone();

            textbox.connect_changed(move | text | {
                if !format!("{:?}", container_clone.children()).contains("GtkScrolledWindow") {
                    if !format!("{:?}", scrollable_clone.children()).contains("GtkViewport") {
                        scrollable_clone.add(&list);

                    }

                    container_clone.add(&scrollable_clone);
                    container_clone.show_all();
                }

                list.foreach(| i | list.remove(i));

                Self::get_apps("/usr/share/applications")
                    .into_iter()
                    .filter(| i |
                        i.name.to_lowercase().contains(&format!("{}", text.text().to_lowercase())) ||
                        i.exec.contains(&format!("{}", text.text().to_lowercase()))
                    )
                    .for_each(| i | Self::add(&i, &list, &config_clone));

                if text.text() == "" || list.children().len() == 0 {
                    container_clone.remove(&scrollable_clone);

                }

                list.show_all();

                let size = match list.children().len() as i32 {
                    n if n <= 2 => 0,
                    n if n <= 3 => 75,
                    n if n <= 5 => 100,
                    _ => self.config.container.max_height,
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

        self.app.run_with_args(&[""]);
    }

    fn exec(args: Vec<&str>) {
        Command::new(args[0])
            .args(&args[1..])
            .spawn()
            .expect("Failed to run command");

        exit(0);
    }

    fn add(app: &crate::utils::types::App, list: &ListBox, config: &Config) {
        let container = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .halign(Align::Center)
            .build();
        
        let image = Image::builder()
            .icon_name(&app.icon)
            .icon_size(IconSize::Button)
            .build();

        let mut name = Label::new(Some(&format!("{}", &app.name)));

        if app.generic != "" {
            name = Label::new(Some(&format!("{} [{}]", &app.name, &app.generic)));
        
        }

        if config.window.icons {
            container.add(&image);
        
        }

        container.add(&name);

        list.add(&container);
    }

    fn select(list: &ListBox, apps: &Vec<crate::utils::types::App>) {
        let label: String = list
            .selected_row()
            .expect("Failed to get row")
            .child()
            .expect("Failed to get child")
            .downcast::<Box>()
            .expect("Failed to downcast")
            .children()
            .into_iter()
            .filter(| i | format!("{:?}", i).contains("GtkLabel"))
            .collect::<Vec<gtk::Widget>>()[0]
            .property_value("label")
            .get()
            .unwrap();

        let app = &apps
            .clone()
            .into_iter()
            .filter(| i | i.name == label.split(" [").collect::<Vec<&str>>()[0])
            .collect::<Vec<crate::utils::types::App>>()[0];

        let args = app
            .exec
            .split(" ")
            .collect::<Vec<&str>>();

        Self::exec(args);
    }

    pub fn get_apps(path: &str) -> Vec<crate::utils::types::App> {
        let mut apps: Vec<crate::utils::types::App> = vec![];

        for file in read_dir(path).expect("Failed to read directory") {
            let mut app = crate::utils::types::App {
                name: String::from(""),
                generic: String::from(""),
                exec: String::from(""),
                icon: String::from(""),
            };

            let file_path = file
                .expect("Failed to read file")
                .path();

            let content = read_to_string(file_path)
                .expect("Failed to read file");

            let raw_lines = content
                .split("\n")
                .collect::<Vec<&str>>();

            for i in &raw_lines {
                if i.contains("Desktop Action") { break }

                if i.starts_with("Name=") {
                    app.name = i
                        .split("=")
                        .collect::<Vec<&str>>()[1]
                        .to_string();
                }

                if i.starts_with("GenericName=") {
                    app.generic = i
                        .split("=")
                        .collect::<Vec<&str>>()[1]
                        .to_string();
                }

                if i.starts_with("Exec=") {
                    app.exec = i
                        .split("=")
                        .collect::<Vec<&str>>()[1]
                        .to_string();
                }

                if i.starts_with("Icon=") {
                    app.icon = i
                        .split("=")
                        .collect::<Vec<&str>>()[1]
                        .to_string();
                }
            }
            
            apps.push(app);
        }

        apps
    }
}
