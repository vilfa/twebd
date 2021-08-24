pub mod config;
pub mod logger;
pub mod out;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        match log_level {
            0 => Self::Off,
            1 => Self::Error,
            2 => Self::Warning,
            3 => Self::Info,
            4 => Self::Debug,
            _ => Self::default(),
        }
    }
}

pub enum Color {
    None,
    Red,
    Orange,
    Blue,
}

impl Color {
    pub fn color(color: &Color, msg: &str) -> String {
        match color {
            Color::Red => format!("\u{001b}[31;1m{}\u{001b}[0m", msg),
            Color::Orange => format!("\u{001b}[33;1m{}\u{001b}[0m", msg),
            Color::Blue => format!("\u{001b}[34;1m{}\u{001b}[0m", msg),
            Color::None => format!("\u{001b}[0m{}\u{001b}[0m", msg),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::None
    }
}
