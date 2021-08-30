use super::worker::Job;
use crate::log::{LogRecord, LoggerConfigureMessage};

pub enum Message {
    Job(Job),
    Log(LogRecord),
    LogConfigure(LoggerConfigureMessage),
    Terminate,
}
