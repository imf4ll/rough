use gtk::prelude::*;
use crate::utils::types::Config;
use gtk::{
    ApplicationWindow,
    cairo,
    gdk,
};

pub fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}

pub fn draw(_window: &ApplicationWindow, ctx: &cairo::Context, config: &Config) -> Inhibit {
    let color = &config.window.background_color
        .split(",")
        .into_iter()
        .map(| n | n.trim().parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    ctx.set_source_rgba(color[0], color[1], color[2], config.window.opacity);
    ctx.set_operator(cairo::Operator::Screen);

    ctx.paint()
        .expect("Failed to set window as transparent");

    Inhibit(false)
}
