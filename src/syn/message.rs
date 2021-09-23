use crate::syn::Job;

pub enum Message {
    Job(Job),
    // Log(LogRecord),
    // LogConfigure(LoggerConfigureMessage),
    Terminate,
}
