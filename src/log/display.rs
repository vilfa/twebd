use crate::log::native::{LogLevel, LogRecord};
use std::fmt::{Display, Formatter, Result};

pub enum Color {
    None,
    Red,
    Yellow,
    Blue,
    Green,
}

impl Color {
    pub fn color(color: &Color, msg: &str) -> String {
        match color {
            Color::Red => format!("\u{001b}[31;1m{}\u{001b}[0m", msg),
            Color::Yellow => format!("\u{001b}[33;1m{}\u{001b}[0m", msg),
            Color::Blue => format!("\u{001b}[34;1m{}\u{001b}[0m", msg),
            Color::Green => format!("\u{001b}[32;1m{}\u{001b}[0m", msg),
            Color::None => format!("\u{001b}[0m{}\u{001b}[0m", msg),
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for LogRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "#{}#{}#{}", self.timestamp, self.log_level, self.msg)
    }
}
