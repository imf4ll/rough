mod transparency;

use std::process::{Command, exit};
use std::fs::{read_dir, read_to_string};

use crate::utils::types::Config;
use crate::app::transparency::*;
use crate::styling::Provider;
use crate::modules;

use gtk::prelude::*;
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

            let container = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(0)
                .valign(Align::Center)
                .build();

            let search = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(0)
                .margin(5)
                .margin_top(0)
                .margin_bottom(10)
                .build();

            let magnifier = Image::builder()
                .icon_name("system-search")
                .icon_size(IconSize::Button)
                .margin_start(6)
                .build();

            let textbox = Entry::new();

            textbox
                .style_context()
                .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

            textbox.set_placeholder_text(Some("Type your command or app"));
            textbox.set_margin(0);
            textbox.set_margin_start(5);
            textbox.set_margin_end(5);
            textbox.set_app_paintable(true);
            textbox.set_hexpand(true);
            textbox.set_xalign(0.015);

            search.add(&magnifier);
            search.add(&textbox);

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
                .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

            let scrollable = ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::External)
                .margin_top(self.config.list.margin_top)
                .build();

            let top_bar = Box::builder()
                .halign(Align::End)
                .spacing(10)
                .margin_end(10)
                .margin_top(8)
                .hexpand(false)
                .build();

            if self.config.modules.video_downloader.enable || self.config.modules.weather.enable {
                container.add(&top_bar);

            }

            let container_clone = container.clone();
            let scrollable_clone = scrollable.clone();
            let provider_clone = provider.clone();
            let list_clone = list.clone();
            
            textbox.connect_changed(move | text | {
                list.set_widget_name("apps");

                list.set_halign(Align::Start);
                list.set_width_request(self.config.window.width);

                if text.text().as_str() != "" {
                    list.connect_key_press_event(| list, event | {
                        if event.keycode() == Some(36) && list.widget_name() == "apps" {
                            Self::select(list, &Self::get_apps("/usr/share/applications"));

                        }

                        Inhibit(false)
                    });

                    list.connect_button_press_event(| list, event| {
                        if event.button() == 1 && list.widget_name() == "apps" {
                            list.connect_row_selected(| list, _ | {
                                Self::select(list, &Self::get_apps("/usr/share/applications"));
                            });
                        }

                        Inhibit(false)
                    });
                }

                if !format!("{:?}", container_clone.children()).contains("GtkScrolledWindow") {
                    if !format!("{:?}", scrollable_clone.children()).contains("GtkViewport") {
                        scrollable_clone.add(&list);

                    }

                    container_clone.add(&scrollable_clone);
                    container_clone.show_all();
                }

                list.foreach(| i | list.remove(i));

                if self.config.modules.calc {
                    modules::calc::calc(&text, &provider, &list);
                
                }
                
                Self::get_apps("/usr/share/applications")
                    .into_iter()
                    .filter(| i |
                        i.name.to_lowercase().contains(&format!("{}", text.text().to_lowercase())) ||
                        i.exec.contains(&format!("{}", text.text().to_lowercase()))
                    )
                    .for_each(| i | Self::add(&i, &list, &provider));

                if text.text() == "" || list.children().len() == 0 {
                    container_clone.remove(&scrollable_clone);

                }

                list.show_all();

                let size = match list.children().len() as i32 {
                    n if n <= 1 => 0,
                    n if n <= 2 => 80,
                    n if n <= 3 => 100,
                    n if n <= 5 => 150,
                    _ => self.config.container.max_height,
                };

                scrollable_clone.set_height_request(size);
                scrollable_clone.show_all();
            });

            container.add(&search);

            let config_clone = self.config.clone();

            modules::modules(
                &config_clone,
                &container,
                &scrollable,
                &textbox,
                &list_clone,
                &provider_clone,
                &top_bar,
            );

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

    fn add(app: &crate::utils::types::App, list: &ListBox, provider: &gtk::CssProvider) {
        let container = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .halign(Align::Start)
            .valign(Align::Center)
            .build();

        let text = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(5)
            .valign(Align::Start)
            .build();
        
        let image = Image::builder()
            .icon_name(&app.icon)
            .icon_size(IconSize::Dnd)
            .build();

        let name = Label::new(Some(&format!("{}", &app.name)));

        name
            .set_halign(Align::Start);

        name
            .style_context()
            .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

        name
            .style_context()
            .add_class("title");

        container.add(&image);

        text.add(&name);

        if app.generic != "" {
            let generic = Label::new(Some(&format!("{}", &app.generic)));

            generic
                .set_halign(Align::Start);

            generic
                .style_context()
                .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

            generic
                .style_context()
                .add_class("generic");

            text.add(&generic);
        
        } else {
            let exec = Label::new(Some(&format!("{}", &app.exec)));

            exec
                .set_halign(Align::Start);

            exec
                .style_context()
                .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

            exec
                .style_context()
                .add_class("generic");

            text.add(&exec);
        }

        container.add(&text);

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
            .filter(| i | format!("{:?}", i).contains("GtkBox"))
            .collect::<Vec<gtk::Widget>>()[0]
            .clone()
            .downcast::<Box>()
            .expect("Failed to downcast")
            .children()[0]
            .clone()
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
                    let mut exec = i
                        .split("=")
                        .collect::<Vec<&str>>()[1]
                        .to_string();

                    if exec.contains("%") {
                        let args = exec
                            .split(" ")
                            .collect::<Vec<&str>>();
                        
                        exec = args[..args.len() - 1]
                            .join(" ");
                    }

                    app.exec = exec;
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
