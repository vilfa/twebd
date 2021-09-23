pub mod config;
pub mod default;
pub mod display;
pub mod native;

pub use config::{Config, Configure, LoggerConfigureMessage};
pub use display::Color;
pub use native::{LogLevel, LogRecord};

use std::{
    io::{stderr, stdout, Write},
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
    pub fn log(&self, record: LogRecord) {
        if self.loggable(&record) {
            let _lock = self.out_lock.lock().unwrap();
            match record.log_level {
                LogLevel::Error => {
                    let stderr = stderr();
                    let mut stderr_lock = stderr.lock();
                    let _ = write!(&mut stderr_lock, "{}", record);
                }
                _ => {
                    let stdout = stdout();
                    let mut stdout_lock = stdout.lock();
                    let _ = write!(&mut stdout_lock, "{}", record);
                }
            }
        }
    }
    fn loggable(&self, record: &LogRecord) -> bool {
        record.log_level as u8 <= self.log_level as u8
    }
}
