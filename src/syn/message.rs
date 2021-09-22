use crate::{
    log::{config::LoggerConfigureMessage, native::LogRecord},
    syn::worker::Job,
};

pub enum Message {
    Job(Job),
    Log(LogRecord),
    LogConfigure(LoggerConfigureMessage),
    Terminate,
}
