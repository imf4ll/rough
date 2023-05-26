use std::env::var;
use std::fs::{read_to_string, metadata, File};
use std::io::Write;
use std::time::SystemTime;
use chrono::prelude::*;

use serde::{Serialize, Deserialize};

use gtk::prelude::*;
use gtk::{
    Box,
    CssProvider,
    Orientation,
    Image,
    Label,
    Align,
    STYLE_PROVIDER_PRIORITY_USER
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub weather: Vec<Weather>,
    pub main: Main,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub main: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32
}

pub fn weather(
        key: String, city: String, cache_time: u128, units: String,
        provider: &CssProvider, top_bar: &Box,
    ) {
    let data = cache(city, key, cache_time, units);

    let weather_box = Box::builder()
        .spacing(5)
        .orientation(Orientation::Horizontal)
        .halign(Align::End)
        .build();

    let image = Image::builder()
        .icon_name(weather_icon(&data.weather[0].main))
        .build();

    let label = Label::new(Some(&format!("{}°", data.main.temp as u32)));

    label
        .style_context()
        .add_provider(provider, STYLE_PROVIDER_PRIORITY_USER);

    weather_box.add(&image);
    weather_box.add(&label);

    top_bar.add(&weather_box);
    top_bar.show_all();
}

fn weather_icon(weather_name: &String) -> &'static str {
    let hours = get_hours();

    match weather_name.as_str() {
        "Clear" => if hours >= 18 || hours <= 6 { "weather-clear-night" } else { "weather-clear" },
        "Snow" => "weather-snow",
        "Thunderstorm" => "weather-storm",
        "Clouds" => if hours >= 18 || hours <= 6 { "weather-few-clouds-night" } else { "weather-few-clouds" },
        "Fog" => "weather-fog",
        "Rain" => "weather-showers",
        "Drizzle" => "weather-showers-scattered",
        "Haze" => "weather-overcast",
        _ => "weather-severe-alert"
    }
}

fn get(city: String, key: String, units: String) -> Response {
    reqwest::blocking::get(format!("https://api.openweathermap.org/data/2.5/weather?q={}&units={}&appid={}", city, units, key))
        .expect("Failed to get weather data.")
        .json::<Response>()
        .expect("Failed to parse JSON.")
}

fn cache(city: String, key: String, cache_time: u128, units: String) -> Response {
    let mut path = var("HOME").expect("Failed to get HOME env variable.");

    path.push_str("/.config/rough/weather-cache.json");

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get now duration.")
        .as_millis();

    let md = match metadata(&path) {
        Ok(m) => m,
        Err(_) => return create_cache(city, key, path, units),
    };

    let time = md
        .modified()
        .expect("Failed to get time duration.")
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get timestamp.")
        .as_millis();

    if (now - time) / 60000 >= cache_time {
        return create_cache(city, key, path, units);

    }

    let file = read_to_string(&path)
        .expect("Failed to read file.");

    serde_json::from_str::<Response>(&file)
        .expect("Failed to serialize.")
}

fn create_cache(city: String, key: String, path: String, units: String) -> Response {
    let data = get(city, key, units);
    
    let mut file_cache = File::create(&path)
        .expect("Failed to create cache file.");

    let buf = serde_json::to_string(&data)
        .expect("Failed to serialize.");

    file_cache
        .write_all(&buf.as_bytes())
        .expect("Failed to create cache.");

    data
}

fn get_hours() -> i64 {
    Local::now()
        .format("%H")
        .to_string()
        .parse::<i64>()
        .expect("Failed to get hours.")
}
