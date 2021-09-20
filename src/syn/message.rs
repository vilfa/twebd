use crate::{
    log::{LogRecord, LoggerConfigureMessage},
    syn::worker::Job,
};

pub enum Message {
    Job(Job),
    Log(LogRecord),
    LogConfigure(LoggerConfigureMessage),
    Terminate,
}
