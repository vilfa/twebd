static LOGGER: app::Logger = app::Logger;
fn init_logger(log_level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}

extern crate log;
extern crate mio;
extern crate regex;
extern crate rustls;
extern crate rustls_pemfile;

pub mod app;
pub mod cli;
pub mod net;
pub mod srv;
pub mod syn;
pub mod web;

pub use cli::run;
