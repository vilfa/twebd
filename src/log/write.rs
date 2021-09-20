use crate::log::LogRecord;
use std::io::{Error, Write};

pub trait Writer {
    fn write<W>(&self, record: LogRecord, writer: &mut W) -> Result<(), Error>
    where
        W: Write + Sized;
}
