use crate::log::LogRecord;

pub trait Backlog {
    fn backlog(&self) -> Vec<LogRecord>;
}
