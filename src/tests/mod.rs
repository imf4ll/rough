#![allow(unused_imports)]

mod utils;

use gtk::Application;
use crate::app::App;
use crate::config;
use crate::styling;
use utils::type_of;

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

    let style = styling::Provider {
        config: crate::config::parse(),
    }
        .new();

    assert_eq!("gtk::auto::css_provider::CssProvider", type_of(&style));
}
