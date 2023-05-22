pub mod calc;
mod weather;
mod news;

use std::fs::read_to_string;
use std::io::Write;
use std::fs::File;
use std::env::var;

use crate::utils::types::Config;

use serde_json;

struct List {
    modules: Vec<Module>
}

struct Module {
    name: String,
    description: String,
}

use gtk::{
    Box,
    ScrolledWindow,
    Entry,
    ListBox,
    CssProvider,
};

pub fn modules(
    config: &Config, container: &Box, scrollable: &ScrolledWindow,
    textbox: &Entry, list: &ListBox, provider: &CssProvider,
) {
    if config.modules.news.enable {
        let cfg = config.modules.news.clone();

        self::news::news(
            cfg.key,
            cfg.region,
            cfg.browser,
            container,
            scrollable,
            textbox,
            list,
            provider,
            cfg.cache_time
        );
    }

    if config.modules.weather.enable {
        let cfg = config.modules.weather.clone();

        self::weather::weather(
            cfg.key,
            cfg.city,
            cfg.cache_time,
            cfg.units,
            container,
            provider
        );
    }
}

pub fn show_modules(config: &Config) {
    let modules = &config.modules;
    
    let mut list: List = List {
        modules: vec![
            Module {
                name: String::from("\x1b[1;31m[✘] Calculator\x1b[m"),
                description: String::from("Calculates simple strings. (Ex: 1 + 1, 2 / 1)")
            },
            Module {
                name: String::from("\x1b[1;31m[✘] Temperature\x1b[m"),
                description: String::from("Shows current weather and temperature.")
            },
            Module {
                name: String::from("\x1b[1;31m[✘] News\x1b[m"),
                description: String::from("Shows last news.")
            }
        ]
    };

    if modules.calc {
        list.modules[0].name = String::from("\x1b[1;32m[🗸] Calculator\x1b[m");

    }

    if modules.weather.enable {
        list.modules[1].name = String::from("\x1b[1;32m[🗸] Weather\x1b[m");

    }

    if modules.news.enable {
        list.modules[2].name = String::from("\x1b[1;32m[🗸] News\x1b[m");

    }

    println!("Modules:");

    list.modules
        .into_iter()
        .for_each(| i | println!("  {}: {}", i.name, i.description));
}

pub fn move_value(key: &String, value: bool) {
    let mut path = var("HOME")
        .expect("Failed to get HOME env variable.");
    
    path.push_str("/.config/rough/config.json");

    let mut config = serde_json::from_str::<Config>(&read_to_string(&path).expect("Failed to open config file."))
        .expect("Failed to get config file.");

    match key.as_str() {
        "calc" => config.modules.calc = value,
        "weather" => config.modules.weather.enable = value,
        "news" => config.modules.news.enable = value,
        _ => panic!("Invalid input, valid modules: calc, weather, news."),
    }

    let mut config_file = File::create(&path)
        .expect("Failed to open config file.");

    config_file.write_all(serde_json::to_string::<Config>(&config).expect("Failed to write config file.").as_bytes())
        .expect("Failed to write config file.");
}

pub fn enable_module(module: String) {
    move_value(&module, true);

    println!("\x1b[1;32m🗸 {module} enabled successfully.\x1b[m");
}

pub fn disable_module(module: String) {
    move_value(&module, false);

    println!("\x1b[1;31m✘ {module} disabled successfully.\x1b[m");
}
