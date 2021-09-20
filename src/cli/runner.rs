use crate::{
    cli::parser,
    log::{
        logger::{Log, Logger},
        LogLevel, LogRecord,
    },
    srv::server::{Server, ServerError},
};

pub fn run() -> Result<(), ServerError> {
    let matches = parser::parse_args();
    match parser::parse_matches(&matches) {
        Ok((opts, backlog)) => {
            let server = Server::new(opts)?;
            server.log_all(&backlog);
            server.listen();
        }
        Err(e) => Logger::new().log(LogRecord::new(LogLevel::Error, format!("{:?}", e))),
    }
    Ok(())
}
