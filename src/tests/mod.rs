#![allow(unused_imports)]

mod utils;

use crate::config;
use crate::styling;
use crate::apps;
use utils::type_of;

// Apps
#[test]
fn valid_apps_vector() {
    let apps_vec = apps::get("/usr/share/applications");

    assert_ne!(0, apps_vec.len());
}

// Config
#[test]
fn valid_config() {
    let config = config::parse();

    assert_ne!(0, config.window.width);
}

// Styling
#[test]
fn valid_style() {
    gtk::init()
        .expect("Failed to initialize GTK");

    let style = styling::provider(&config::parse());

    assert_eq!("gtk::auto::css_provider::CssProvider", type_of(&style));
}
