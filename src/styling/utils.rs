use crate::utils::types::Config;

pub fn create_transparent_css(config: &Config) -> String {
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
            background: transparent;
        }}

        list {{
            background: transparent;
        }}
    ",
        config.window.font_color,
        config.window.border_color,
    )
}

pub fn create_default_css(config: &Config) -> String {
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
        }}
    ",
        config.window.font_color,
        config.window.border_color,
    )
}
