mod utils;

use gtk::prelude::*;
use gtk::CssProvider;
use crate::utils::types::Config;
use utils::*;

pub fn provider(config: &Config) -> CssProvider {
    let mut css: String = create_default_css(config);

    if config.window.opacity < 1.0 {
        css = create_transparent_css(config);
    
    }

    let provider = gtk::CssProvider::new();
    
    provider.load_from_data(css.as_bytes()).expect("Failed to read custom CSS");
    
    provider
}
