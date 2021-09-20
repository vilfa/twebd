use crate::log::{Color, LogLevel};
use std::{array::IntoIter, collections::HashMap, convert::From, iter::FromIterator};

pub struct Config {
    pub show_timestamp: bool,
    pub show_log_level: bool,
    pub timestamp_format: String,
    pub colors: HashMap<LogLevel, Color>,
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

pub trait Configure {
    fn set_log_level(&mut self, log_level: LogLevel);
    fn show_timestamp(&mut self, show: bool);
    fn show_log_level(&mut self, show: bool);
}
