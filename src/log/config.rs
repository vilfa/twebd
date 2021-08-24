use super::{Color, LogLevel};
use std::{array::IntoIter, collections::HashMap, convert::From, iter::FromIterator};

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
            timestamp_format: String::from("%H:%M:%S.%3f%z"),
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

pub trait Configure {
    fn log_level(&mut self, log_level: LogLevel);
}
