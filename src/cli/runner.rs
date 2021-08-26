use super::{parser, CliOpt};
use crate::srv::server::Server;

pub fn run() -> Server {
    let matches = parser::parse_args();
    let opts: Vec<CliOpt> = vec![
        parser::parse_matches_required(&matches),
        parser::parse_matches_optional(&matches),
    ]
    .into_iter()
    .flatten()
    .collect();
    run_with_opts(opts)
}

fn run_with_opts(opts: Vec<CliOpt>) -> Server {
    Server::new(opts)
}
