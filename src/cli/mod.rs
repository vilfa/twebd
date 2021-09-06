pub mod parser;
pub mod runner;

use crate::{log::LogLevel, net::dproto::DataProtocol};
use std::{net::IpAddr, path::PathBuf, process};

#[derive(Debug, Clone)]
pub enum CliOpt {
    Address(IpAddr),
    Port(u16),
    Protocol(DataProtocol),
    Directory(PathBuf),
    Verbosity(LogLevel),
    Threads(usize),
    ShowTimestamp(bool),
    ShowLoglevel(bool),
}

pub fn exit(exit_code: i32) -> ! {
    process::exit(exit_code)
}
