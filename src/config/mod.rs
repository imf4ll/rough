use std::fs::read_to_string;
use std::env::var;

use serde_json;
use crate::utils::types::Config;

pub fn parse() -> Config {
    let path = match var("HOME") {
        Ok(path) => path,
        Err(_) => return Config {
            window_width: 600,
            box_height: 200,
        },
    };

    let content = match read_to_string(&format!("{}/.config/rough/config.json", path)) {
        Ok(text) => text,
        Err(_) => return Config {
            window_width: 600,
            box_height: 200,
        },
    };

    serde_json::from_str::<Config>(&content)
        .expect("Failed to read config file")
}
