use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub generic: String,
    pub exec: String,
}

// CONFIGURATION
#[derive(Deserialize, Debug)]
pub struct Config {
    pub window: Window,
    pub container: Container,
    pub textbox: TextBox,
    pub list: List,
}

#[derive(Deserialize, Debug)]
pub struct Window {
    pub width: i32,
    pub border_width: i32,
    pub border_color: String,
}

#[derive(Deserialize, Debug)]
pub struct Container {
    pub max_height: i32,
}

#[derive(Deserialize, Debug)]
pub struct TextBox {
    pub margin: i32,
}

#[derive(Deserialize, Debug)]
pub struct List {
    pub margin_top: i32,
}
