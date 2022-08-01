use crate::utils::types::Config;

pub fn create_css(config: &Config) -> String {
    format!("
        * {{
            color: {};
        }}

        window {{
            border: 1px solid {};
        }}

        entry {{
            border: none;
            font-weight: 500;
            border-radius: 0;
            padding: 0;
            {}
        }}

        list {{
            {}
        }}
    ",
        config.window.font_color,
        config.window.border_color,
        enabled(config.textbox.transparent, config.window.opacity),
        enabled(config.list.transparent, config.window.opacity)
    )
}

fn enabled(is: bool, value: f64) -> String {
    if is && value < 1.0 {
        "background: transparent;".to_string()
    
    } else {
        "".to_string()
    
    }
}
