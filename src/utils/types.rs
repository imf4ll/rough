use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub generic: String,
    pub exec: String,
    pub icon: String,
}

// CONFIGURATION
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub window: Window,
    pub container: Container,
    pub textbox: TextBox,
    pub list: List,
    pub modules: Modules,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Window {
    pub width: i32,
    pub border_width: i32,
    pub border_color: String,
    pub opacity: f64,
    pub background_color: String,
    pub font_color: String,
    pub font: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    pub max_height: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextBox {
    pub transparent: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub margin_top: i32,
    pub transparent: bool,
    pub inline: bool,
    pub icons: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modules {
    pub calc: bool,
    pub weather: Weather,
    pub news: News,
    pub video_downloader: VideoDownloader,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
    pub enable: bool,
    pub key: String,
    pub city: String,
    pub cache_time: u128,
    pub units: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct News {
    pub enable: bool,
    pub region: String,
    pub key: String,
    pub browser: String,
    pub cache_time: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoDownloader {
    pub enable: bool,
    pub path: String,
}
