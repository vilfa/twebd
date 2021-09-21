use crate::log::LogRecord;

pub trait Backlog {
    fn add_backlog(&mut self, v: LogRecord);
    fn backlog(&self) -> Vec<LogRecord>;
}
