use chrono::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        match log_level {
            0 => Self::Off,
            1 => Self::Error,
            2 => Self::Warning,
            3 => Self::Info,
            4 => Self::Debug,
            _ => Self::default(),
        }
    }
}

#[derive(Clone)]
pub struct LogRecord {
    pub timestamp: DateTime<Local>,
    pub log_level: LogLevel,
    pub msg: String,
}

impl LogRecord {
    pub fn new(log_level: LogLevel, msg: String) -> LogRecord {
        LogRecord {
            timestamp: Local::now(),
            log_level,
            msg,
        }
    }
}
