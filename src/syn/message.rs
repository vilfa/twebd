use super::worker::Job;
use crate::log::{LogLevel, LoggerConfigureMessage};

pub enum Message {
    Job(Job),
    Log(LogLevel, String),
    LogConfigure(LoggerConfigureMessage),
    Terminate,
}
