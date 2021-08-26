pub mod parser;
pub mod runner;

use crate::{log::LogLevel, net::dproto::DataProtocol};
use std::{net::IpAddr, process};

#[derive(Debug, Copy, Clone)]
pub enum CliOpt {
    Address(IpAddr),
    Port(u16),
    Protocol(DataProtocol),
    Verbosity(LogLevel),
    Threads(u8),
}

pub fn exit(exit_code: i32) -> ! {
    process::exit(exit_code)
}
