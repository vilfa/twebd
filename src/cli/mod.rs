pub mod parser;
pub mod runner;

use crate::{log::native::LogLevel, net::DataProtocol};
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

pub trait Build<T, U, E: Sized> {
    fn from(opts: Vec<CliOpt>) -> T;
    fn build(&self) -> std::result::Result<U, E>;
}

pub trait Other {
    fn add_other(&mut self, o: CliOpt);
    fn other(&self) -> Vec<CliOpt>;
}
