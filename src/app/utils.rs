use std::process::{Command, exit};

use gtk::prelude::*;
use crate::utils::types::{App, Config};
use gtk::{
    ListBox,
    Label,
    ApplicationWindow,
    cairo,
    gdk,
    Box,
    Orientation,
    Image,
    IconSize,
    Align,
};

use gtk::glib::object::Cast;

pub fn run(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .expect("Failed to run command");

    exit(0);
}

pub fn add(app: &App, list: &ListBox) {
    let container = Box::new(Orientation::Horizontal, 0);
    
    container.set_halign(Align::Center);

    let image: Image;
    let name: Label;

    if app.generic != "" {
        name = Label::new(Some(&format!("{} [{}]", &app.name, &app.generic)));
    
    } else {
        name = Label::new(Some(&format!("{}", &app.name)));

    }

    if app.icon != "" {
        image = Image::new();

        image.set_from_icon_name(Some(&app.icon), IconSize::Button);
    
        container.add(&image);
        container.add(&name);
    
    } else {
        container.add(&name);

    }

    list.add(&container);
}

pub fn select(list: &ListBox, apps: &Vec<App>) {
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
        .collect::<Vec<App>>()[0];

    let args = app
        .exec
        .split(" ")
        .collect::<Vec<&str>>();

    run(args);
}

pub fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}

pub fn draw(_window: &ApplicationWindow, ctx: &cairo::Context, config: &Config) -> Inhibit {
    let color = &config.window.background_color
        .split(",")
        .into_iter()
        .map(| n | n.trim().parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    ctx.set_source_rgba(color[0], color[1], color[2], config.window.opacity);
    ctx.set_operator(cairo::Operator::Screen);

    ctx.paint()
        .expect("Failed to set window as transparent");

    Inhibit(false)
}
