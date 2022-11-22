pub mod default;
pub mod err;
pub mod parse;
pub mod run;

pub use parse::{parse_args, parse_matches, CliConfig};
pub use run::run;

use crate::net::DataProtocol;
use std::{net::IpAddr, path::PathBuf};

#[derive(Debug, Clone)]
pub enum CliOpt {
    Address(IpAddr),
    Port(u16),
    Protocol(DataProtocol),
    Directory(PathBuf),
    Verbosity(log::LevelFilter),
    Threads(usize),
    ShowTimestamp(bool),
    ShowLoglevel(bool),
    Https(bool),
    HttpsCert(Option<PathBuf>),
    HttpsPrivKey(Option<PathBuf>),
}

pub trait Build<T, V, E>
where
    T: Sized,
    V: Sized,
    E: Sized,
{
    fn new(opts: Vec<CliOpt>) -> T;
    fn build(&self) -> std::result::Result<V, E>;
}

pub trait Other {
    fn add_other(&mut self, o: CliOpt);
    fn other(&self) -> Vec<CliOpt>;
}
