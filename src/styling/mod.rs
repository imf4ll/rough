use gtk::prelude::*;
use gtk::CssProvider;
use crate::utils::types::Config;

pub fn provider(config: &Config) -> CssProvider {
    let css = format!("
        window {{
            border: 1px solid {};
        }}
    ", config.window.border_color);

    let provider = gtk::CssProvider::new();
    
    provider.load_from_data(css.as_bytes()).expect("Failed to read custom CSS");
    
    provider
}
