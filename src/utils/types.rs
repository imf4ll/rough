use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub window_width: i32,
    pub box_height: i32,
}

#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub generic: String,
    pub exec: String,
}
