use super::LogLevel;
use std::io::{Error, Write};

pub trait Writer {
    fn write<W: Write + Sized>(
        &self,
        log_level: LogLevel,
        msg: &str,
        writer: &mut W,
    ) -> Result<(), Error>;
}
