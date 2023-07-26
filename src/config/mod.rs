use std::fs::read_to_string;
use std::env::var;

use serde_json;

use crate::utils::types::{
    Config,
    Window,
    Container,
    TextBox,
    List,
    Modules,
    Weather,
    News,
    VideoDownloader,
};

pub fn parse() -> Config {
    let default_config = Config {
        window: Window {
            width: 600,
            border_width: 6,
            border_color: String::from("#252525"),
            opacity: 1.0,
            background_color: String::from("255, 255, 255"),
            font_color: String::from("#FFFFFF"),
            font: String::from(""),
        },
        container: Container {
            max_height: 200,
        },
        textbox: TextBox {
            transparent: false
        },
        list: List {
            margin_top: 6,
            transparent: false,
            inline: false,
            icons: true,
        },
        modules: Modules {
            calc: true,
            weather: Weather {
                enable: false,
                key: String::new(),
                city: String::new(),
                cache_time: 30,
                units: String::from("metric")
            },
            news: News {
                enable: false,
                key: String::new(),
                region: String::new(),
                browser: String::new(),
                cache_time: 60
            },
            video_downloader: VideoDownloader {
                enable: false,
                path: String::new()
            }
        },
    };

    let path = match var("HOME") {
        Ok(path) => path,
        Err(_) => return default_config,
    };

    let content = match read_to_string(&format!("{}/.config/rough/config.json", path)) {
        Ok(text) => text,
        Err(_) => return default_config,
    };

    match serde_json::from_str::<Config>(&content) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("\x1b[31m[X] Failed to load configuration file, using default config...\x1b[m");

            default_config
        }
    }
}
