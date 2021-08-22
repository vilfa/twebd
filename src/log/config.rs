use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Config {
    timestamp: bool,
    timestamp_format: String,
    loglevel: bool,
    colors: HashMap<LogLevel, Color>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timestamp: true,
            timestamp_format: String::from("[%H:%M:%S.%3f]"),
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Off,
    Error,
    Warning,
    Info,
    Debug,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Error
    }
}

enum Color {
    None,
    Red,
    Orange,
    Blue,
}
