use super::LogLevel;
use std::io::{Error, Write};

pub trait Writer {
    fn write<W>(&self, log_level: LogLevel, msg: &str, writer: &mut W) -> Result<(), Error>
    where
        W: Write + Sized;
}
