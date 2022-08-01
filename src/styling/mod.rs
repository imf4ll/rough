mod utils;

use gtk::prelude::*;
use gtk::CssProvider;
use crate::utils::types::Config;
use utils::create_css;

pub fn provider(config: &Config) -> CssProvider {
    let css = create_css(config);

    let provider = gtk::CssProvider::new();
    
    provider.load_from_data(css.as_bytes()).expect("Failed to read custom CSS");
    
    provider
}
