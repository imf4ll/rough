use std::process::{Command, exit};

use gtk::prelude::*;
use gtk::{ListBox, Label};
use crate::utils::types::App;

pub fn run(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .expect("Failed to run command");

    exit(0);
}

pub fn add(app: &App, list: &ListBox) {
    if app.generic != "" {
        list.add(&Label::new(Some(&format!("{} ({})", &app.name, &app.generic))));

    } else {
        list.add(&Label::new(Some(&app.name)));

    }
}

pub fn select(list: &ListBox, apps: &Vec<App>) {
    let label: String = list
        .selected_row()
        .expect("Failed to get row")
        .child()
        .expect("Failed to get child")
        .property_value("label")
        .get()
        .unwrap();

    let app = &apps
        .clone()
        .into_iter()
        .filter(| i | i.name == label.split(" ").collect::<Vec<&str>>()[0])
        .collect::<Vec<App>>()[0];

    let args = app
        .exec
        .split(" ")
        .collect::<Vec<&str>>();
    
    run(args);
}
