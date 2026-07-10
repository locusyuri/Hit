use rusty_rich::{Color, Console, Style, Text};

pub const CHECK: &str = "✔";
pub const CROSS: &str = "✘";
pub const ARROW: &str = "▶";
pub const WARN: &str = "⚠";

pub struct Theme {
    pub success: Style,
    pub error: Style,
    pub warn: Style,
    pub info: Style,
    pub step: Style,
    pub dim: Style,
    pub header: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            success: Style::new().color(Color::parse("green").unwrap()),
            error: Style::new().color(Color::parse("red").unwrap()),
            warn: Style::new().color(Color::parse("yellow").unwrap()),
            info: Style::new().color(Color::parse("white").unwrap()),
            step: Style::new().color(Color::parse("cyan").unwrap()),
            dim: Style::new().color(Color::parse("grey50").unwrap()),
            header: Style::new().color(Color::parse("cyan").unwrap()).bold(true),
        }
    }
}

pub fn success(msg: &str) -> Text {
    Text::new(msg).style(Theme::default().success)
}

pub fn error(msg: &str) -> Text {
    Text::new(msg).style(Theme::default().error)
}

pub fn warn(msg: &str) -> Text {
    Text::new(msg).style(Theme::default().warn)
}

pub fn info(msg: &str) -> Text {
    Text::new(msg).style(Theme::default().info)
}

pub fn step(msg: &str) -> Text {
    Text::new(msg).style(Theme::default().step)
}

pub fn dim(msg: &str) -> Text {
    Text::new(msg).style(Theme::default().dim)
}

pub fn bold(msg: &str) -> Text {
    Text::new(msg).style(Style::new().bold(true))
}

pub fn print_success(msg: &str) {
    let mut console = Console::new();
    console.println(&success(msg));
}

pub fn print_error(msg: &str) {
    let mut console = Console::new();
    console.println(&error(msg));
}

pub fn print_warn(msg: &str) {
    let mut console = Console::new();
    console.println(&warn(msg));
}

pub fn print_info(msg: &str) {
    let mut console = Console::new();
    console.println(&info(msg));
}

pub fn print_step(msg: &str) {
    let mut console = Console::new();
    console.println(&step(msg));
}

pub fn header_style() -> Style {
    Theme::default().header
}
