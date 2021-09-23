use crate::log::{Color, Config, LogLevel};
use std::{array::IntoIter, collections::HashMap, iter::FromIterator};

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            show_timestamp: true,
            show_log_level: true,
            timestamp_format: String::from("%H:%M:%S.%3f%z"),
            colors: HashMap::<LogLevel, Color>::from_iter(IntoIter::new([
                (LogLevel::Off, Color::None),
                (LogLevel::Error, Color::Red),
                (LogLevel::Warning, Color::Yellow),
                (LogLevel::Info, Color::Blue),
                (LogLevel::Debug, Color::Green),
            ])),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::None
    }
}
