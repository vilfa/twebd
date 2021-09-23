use crate::{
    log::{LogRecord, LoggerConfigureMessage},
    syn::Job,
};

pub enum Message {
    Job(Job),
    Log(LogRecord),
    LogConfigure(LoggerConfigureMessage),
    Terminate,
}
