use super::{
    config::{Config, Configure},
    out::Writer,
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
    pub fn init() -> Logger {
        Logger::new(LogLevel::default(), Config::default())
    }
    fn new(log_level: LogLevel, config: Config) -> Logger {
        Logger {
            log_level,
            config,
            out_lock: Mutex::new(()),
        }
    }
    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level(log_level);
    }
}

pub trait Log {
    fn enabled(&self) -> bool;
    fn log(&self, log_level: LogLevel, msg: &str);
    fn err(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn info(&self, msg: &str);
}

impl Log for Logger {
    fn enabled(&self) -> bool {
        self.log_level != LogLevel::Off
    }
    fn log(&self, log_level: LogLevel, msg: &str) {
        if self.enabled() {
            let _lock = self.out_lock.lock().unwrap();
            if log_level as u8 <= self.log_level as u8 {
                match log_level {
                    LogLevel::Error => {
                        let stderr = stderr();
                        let mut stderr_lock = stderr.lock();
                        let _ = self.write(log_level, msg, &mut stderr_lock);
                    }
                    _ => {
                        let stdout = stdout();
                        let mut stdout_lock = stdout.lock();
                        let _ = self.write(log_level, msg, &mut stdout_lock);
                    }
                }
            }
        }
    }
    fn err(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
    fn warn(&self, msg: &str) {
        self.log(LogLevel::Warning, msg);
    }
    fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg)
    }
}

impl Configure for Logger {
    fn log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }
}

impl Writer for Logger {
    fn write<W>(&self, log_level: LogLevel, msg: &str, writer: &mut W) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let mut log_msg = String::new();

        if self.config.timestamp {
            let t = Local::now()
                .format(&self.config.timestamp_format)
                .to_string();
            log_msg.push_str(format!("#{}#", &t).as_str());
        }

        if self.config.loglevel {
            log_msg.push_str(
                format!(
                    "#{}#",
                    Color::color(
                        self.config.colors.get(&log_level).unwrap(),
                        format!("{:?}", log_level).as_str()
                    )
                )
                .as_str(),
            );
        }

        log_msg.push_str(format!(" {}\n", &msg).as_str());

        write!(writer, "{}", &log_msg)
    }
}
