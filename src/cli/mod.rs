pub mod parser;
pub mod runner;

use crate::{log::LogLevel, net::dproto::DataProtocol};
use std::net::IpAddr;

#[derive(Debug)]
pub enum CliOpt {
    Address(IpAddr),
    Port(u16),
    Protocol(DataProtocol),
    Verbosity(LogLevel),
}
