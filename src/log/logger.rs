use crate::log::{
    config::{Config, Configure},
    write::Writer,
    Color, LogLevel, LogRecord,
};
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
    fn loggable(&self, record: &LogRecord) -> bool;
    fn log(&self, record: LogRecord);
}

impl Log for Logger {
    fn enabled(&self) -> bool {
        self.log_level != LogLevel::Off
    }
    fn loggable(&self, record: &LogRecord) -> bool {
        record.log_level as u8 <= self.log_level as u8
    }
    fn log(&self, record: LogRecord) {
        if self.enabled() && self.loggable(&record) {
            let _lock = self.out_lock.lock().unwrap();
            match record.log_level {
                LogLevel::Error => {
                    let stderr = stderr();
                    let mut stderr_lock = stderr.lock();
                    let _ = self.write(record, &mut stderr_lock);
                }
                _ => {
                    let stdout = stdout();
                    let mut stdout_lock = stdout.lock();
                    let _ = self.write(record, &mut stdout_lock);
                }
            }
        }
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
    fn write<W>(&self, record: LogRecord, writer: &mut W) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let mut l = String::new();

        if self.config.show_timestamp {
            l.push_str(&format!(
                "#{}#",
                record.timestamp.format(&self.config.timestamp_format)
            ));
        }

        if self.config.show_log_level {
            l.push_str(&format!(
                "#{}#",
                Color::color(
                    self.config.colors.get(&record.log_level).unwrap(),
                    &format!("{:?}", record.log_level)
                )
            ));
        }

        l.push_str(&format!("{}\n", record.msg));

        write!(writer, "{}", &l)
    }
}
