use rusty_rich::{Color, Style};

pub struct Theme {
    pub header: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            header: Style::new().color(Color::parse("cyan").unwrap()).bold(true),
        }
    }
}

pub fn header_style() -> Style {
    Theme::default().header
}
