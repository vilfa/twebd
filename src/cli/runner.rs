use crate::{
    cli::parser,
    log::{
        backlog::Backlog,
        logger::{Log, Logger},
        LogLevel, LogRecord,
    },
    srv::server::Server,
};

pub fn run() {
    let matches = parser::parse_args();
    match parser::parse_matches(&matches) {
        Ok((opts, backlog)) => match Server::new(opts) {
            Ok(server) => {
                server.log_all(&backlog);
                server.log_all(&server.backlog());
                server.listen();
            }
            Err(e) => {
                Logger::new().log(LogRecord::new(LogLevel::Error, format!("{:?}", e)));
            }
        },
        Err(e) => Logger::new().log(LogRecord::new(LogLevel::Error, format!("{:?}", e))),
    }
}
