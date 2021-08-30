use super::parser;
use crate::{
    log::{
        logger::{Log, Logger},
        LogLevel, LogRecord,
    },
    srv::server::Server,
};

pub fn run() {
    let matches = parser::parse_args();
    match parser::parse_matches(&matches) {
        Ok((opts, backlog)) => {
            let server = Server::new(opts);
            for record in backlog {
                server.log(record);
            }
            server.listen();
        }
        Err(e) => Logger::new().log(LogRecord::new(LogLevel::Error, format!("{:?}", e))),
    }
}
