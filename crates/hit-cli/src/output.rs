use owo_colors::OwoColorize;

pub fn success(message: &str) {
    println!("{}", format!("✔ {}", message).green());
}

pub fn error(message: &str) {
    println!("{}", format!("✘ {}", message).red());
}

pub fn warn(message: &str) {
    println!("{}", format!("⚠ {}", message).yellow());
}

pub fn info(message: &str) {
    println!("{}", message);
}

pub fn step(message: &str) {
    println!("{}", format!("▶ {}", message).cyan());
}

pub fn dim(message: &str) {
    println!("{}", message.dimmed());
}

pub fn print_header(text: &str) {
    println!("{}", text.cyan().bold());
}

pub fn print_label(label: &str, value: &str) {
    println!("{}: {}", label.cyan().bold(), value);
}

pub fn print_key_value(key: &str, value: &str) {
    println!("{}: {}", key.cyan().bold(), value);
}