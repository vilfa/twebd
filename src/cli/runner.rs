use crate::{
    cli::parser,
    log::{native::LogLevel, Backlog, Logger},
    // srv::server::Server,
};

pub fn run() {
    // let matches = parser::parse_args();
    // match parser::parse_matches(&matches) {
    //     Ok((opts, backlog)) => match Server::new(opts) {
    //         Ok(server) => {
    //             server.log_all(&backlog);
    //             server.log_all(&server.backlog());
    //             server.listen();
    //         }
    //         Err(e) => {
    //             Logger::new().log(logf!(LogLevel::Error, "{:?}", e));
    //         }
    //     },
    //     Err(e) => Logger::new().log(logf!(LogLevel::Error, "{:?}", e)),
    // }
}
