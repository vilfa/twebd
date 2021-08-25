use super::worker::Job;
use crate::log::{ConfigureMessage, LogLevel};

pub enum Message {
    Job(Job),
    Log(LogLevel, String),
    LogConfigure(ConfigureMessage),
    Terminate,
}
