use crate::log::{Color, LogLevel, Logger};
use std::collections::HashMap;

pub struct Config {
    pub show_timestamp: bool,
    pub show_log_level: bool,
    pub timestamp_format: String,
    pub colors: HashMap<LogLevel, Color>,
}

pub trait Configure {
    fn with(&mut self, conf: Config);
    fn set_log_level(&mut self, log_level: LogLevel);
    fn show_timestamp(&mut self, show: bool);
    fn show_log_level(&mut self, show: bool);
}

impl Configure for Logger {
    fn with(&mut self, conf: Config) {
        self.config = conf;
    }
    fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }
    fn show_timestamp(&mut self, show: bool) {
        self.config.show_timestamp = show;
    }
    fn show_log_level(&mut self, show: bool) {
        self.config.show_log_level = show;
    }
}

pub enum LoggerConfigureMessage {
    SetLogLevel(LogLevel),
    ShowTimestamp(bool),
    ShowLogLevel(bool),
}
