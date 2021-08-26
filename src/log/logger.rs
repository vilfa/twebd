use super::{
    config::{Config, Configure},
    write::Writer,
    Color, LogLevel,
};
use chrono::prelude::*;
use std::{
    io::{stderr, stdout, Error, Write},
    sync::Mutex,
};

pub struct Logger {
    pub log_level: LogLevel,
    pub config: Config,
    out_lock: Mutex<()>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            log_level: LogLevel::default(),
            config: Config::default(),
            out_lock: Mutex::new(()),
        }
    }
}

pub trait Log {
    fn enabled(&self) -> bool;
    fn log(&self, log_level: LogLevel, msg: String);
    fn err(&self, msg: String);
    fn warn(&self, msg: String);
    fn info(&self, msg: String);
    fn debug(&self, msg: String);
}

impl Log for Logger {
    fn enabled(&self) -> bool {
        self.log_level != LogLevel::Off
    }
    fn log(&self, log_level: LogLevel, msg: String) {
        if self.enabled() {
            let _lock = self.out_lock.lock().unwrap();
            if log_level as u8 <= self.log_level as u8 {
                match log_level {
                    LogLevel::Error => {
                        let stderr = stderr();
                        let mut stderr_lock = stderr.lock();
                        let _ = self.write(log_level, &msg, &mut stderr_lock);
                    }
                    _ => {
                        let stdout = stdout();
                        let mut stdout_lock = stdout.lock();
                        let _ = self.write(log_level, &msg, &mut stdout_lock);
                    }
                }
            }
        }
    }
    fn err(&self, msg: String) {
        self.log(LogLevel::Error, msg);
    }
    fn warn(&self, msg: String) {
        self.log(LogLevel::Warning, msg);
    }
    fn info(&self, msg: String) {
        self.log(LogLevel::Info, msg)
    }
    fn debug(&self, msg: String) {
        self.log(LogLevel::Debug, msg);
    }
}

impl Configure for Logger {
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

impl Writer for Logger {
    fn write<W>(&self, log_level: LogLevel, msg: &str, writer: &mut W) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let mut record = String::new();

        if self.config.show_timestamp {
            let t = Local::now()
                .format(&self.config.timestamp_format)
                .to_string();
            record.push_str(&format!("#{}#", t));
        }

        if self.config.show_log_level {
            record.push_str(&format!(
                "#{}#",
                Color::color(
                    self.config.colors.get(&log_level).unwrap(),
                    &format!("{:?}", log_level)
                )
            ));
        }

        record.push_str(&format!("{}\n", msg));

        write!(writer, "{}", &record)
    }
}
