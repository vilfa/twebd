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

impl Default for CliOpt {
    fn default() -> Self {
        match Self {
            CliOpt::Address(_) => todo!(),
            CliOpt::Port(_) => todo!(),
            CliOpt::Protocol(_) => todo!(),
            CliOpt::Directory(_) => todo!(),
            CliOpt::Verbosity(_) => todo!(),
            CliOpt::Threads(_) => todo!(),
            CliOpt::ShowTimestamp(_) => todo!(),
            CliOpt::ShowLoglevel(_) => todo!(),
            CliOpt::Https(_) => todo!(),
            CliOpt::HttpsCert(_) => todo!(),
            CliOpt::HttpsPrivKey(_) => todo!(),
        }
    }
}

pub trait Builder<T, V, E>
where
    T: Sized,
    V: Sized,
    E: Sized,
{
    fn new(opts: Vec<CliOpt>) -> T;
    fn build(&self) -> std::result::Result<V, E>;
    fn add_other(&mut self, o: CliOpt);
    fn other(&self) -> Vec<CliOpt>;
}
