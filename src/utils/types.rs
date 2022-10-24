use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub generic: String,
    pub exec: String,
    pub icon: String,
}

// CONFIGURATION
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub window: Window,
    pub container: Container,
    pub textbox: TextBox,
    pub list: List,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Window {
    pub width: i32,
    pub border_width: i32,
    pub border_color: String,
    pub opacity: f64,
    pub background_color: String,
    pub font_color: String,
    pub font: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Container {
    pub max_height: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextBox {
    pub margin: i32,
    pub transparent: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct List {
    pub margin_top: i32,
    pub transparent: bool,
}
