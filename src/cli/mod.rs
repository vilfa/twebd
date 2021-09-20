pub mod parser;
pub mod runner;

use crate::{log::LogLevel, net::DataProtocol};
use std::{net::IpAddr, path::PathBuf};

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
    Https(bool),
    HttpsCert(PathBuf),
    HttpsPrivKey(PathBuf),
}
