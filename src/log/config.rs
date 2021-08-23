use std::array::IntoIter;
use std::collections::HashMap;
use std::convert::From;
use std::iter::FromIterator;

pub struct Config {
    pub timestamp: bool,
    pub timestamp_format: String,
    pub loglevel: bool,
    pub colors: HashMap<LogLevel, Color>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timestamp: true,
            timestamp_format: String::from("%H:%M:%S.%3f"),
            loglevel: true,
            colors: HashMap::<LogLevel, Color>::from_iter(IntoIter::new([
                (LogLevel::Off, Color::None),
                (LogLevel::Error, Color::Red),
                (LogLevel::Warning, Color::Orange),
                (LogLevel::Info, Color::Blue),
            ])),
        }
    }
}

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
        LogLevel::Info
    }
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        match log_level {
            0 => LogLevel::Off,
            1 => LogLevel::Error,
            2 => LogLevel::Warning,
            3 => LogLevel::Info,
            4 => LogLevel::Debug,
            _ => LogLevel::default(),
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
