use crate::{
    cli::{defaults, parser},
    srv::{log::init_logger, HttpServer, HttpsServer, Server},
};
use log::error;

pub fn run() {
    let _ = init_logger(defaults::loglevel());
    let matches = parser::parse_args();
    match parser::parse_matches(&matches) {
        Ok(cli_config) => {
            log::set_max_level(cli_config.log_level());
            if cli_config.https() {
                HttpsServer::new(cli_config.cli_opts()).listen();
            } else {
                HttpServer::new(cli_config.cli_opts()).listen();
            }
        }
        Err(e) => error!("{:?}", e),
    }
}
