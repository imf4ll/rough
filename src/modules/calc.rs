use eval::eval;

use gtk::prelude::*;
use gtk::{
    Entry,
    Box,
    Orientation,
    ListBox,
    Label,
    Image,
    IconSize,
    Align,
    CssProvider,
};

pub fn calc(text: &Entry, provider: &CssProvider, list: &ListBox) {
    if is_calc_valid(text.text().as_str()) {
        match eval(text.text().as_str()) {
            Ok(_) => {
                let container = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .spacing(10)
                    .halign(Align::Start)
                    .build();

                let image = Image::builder()
                    .icon_name("accessories-calculator")
                    .icon_size(IconSize::Dnd)
                    .build();

                container.add(&image);

                let calc = eval (
                    text
                        .text()
                        .as_str()
                )
                    .unwrap();

                let label = Label::new(Some(&format!("{calc}")));

                label
                    .style_context()
                    .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

                label
                    .style_context()
                    .add_class("title");

                container.add(&label);

                if contains_number(&format!("{calc}")) {
                    list.add(&container);

                }
            },
            Err(_) => {},
        }
    }
}

fn is_calc_valid(text: &str) -> bool {
    text
        .trim()
        .replace(" ", "")
        .bytes()
        .all(| b | matches!(b, b'%'..=b'9'))
    &&
    !text
        .bytes()
        .all(| b | matches!(b, b'a'..=b'z'))
    &&
    !text
        .bytes()
        .all(| b | matches!(b, b'A'..=b'Z'))
}

fn contains_number(text: &str) -> bool {
    text
        .trim()
        .replace(" ", "")
        .bytes()
        .all(| b | matches!(b, b'.'..=b'9'))
}
