use crate::log::config::{Config, LogLevel};
use std::io::{stderr, stdout};
use std::sync::Mutex;

trait Log {
    fn log(&self, log_level: LogLevel, msg: &str);
    fn err(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn info(&self, msg: &str);
}

pub struct Logger {
    log_level: LogLevel,
    config: Config,
    out: Mutex<()>,
}

impl Logger {
    fn new(log_level: LogLevel, config: Config) -> Logger {
        Logger {
            log_level,
            config,
            out: Mutex::new(()),
        }
    }
}

impl Log for Logger {
    fn log(&self, log_level: LogLevel, msg: &str) {}
    fn err(&self, msg: &str) {}
    fn warn(&self, msg: &str) {}
    fn info(&self, msg: &str) {}
}
