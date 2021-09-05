use super::LogRecord;

pub trait Backlog {
    fn backlog(&self) -> Vec<LogRecord>;
}
