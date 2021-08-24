use super::{parser, CliOpt};
use crate::log::logger::{Log, Logger};

pub fn run(log: &mut Logger) {
    let matches = parser::parse_args();
    let opts: Vec<CliOpt> = vec![
        parser::parse_matches_required(log, &matches),
        parser::parse_matches_optional(log, &matches),
    ]
    .into_iter()
    .flatten()
    .collect();
    run_with_opts(log, opts);
}

fn run_with_opts(log: &mut Logger, opts: Vec<CliOpt>) {
    for opt in &opts {
        match opt {
            CliOpt::Address(a) => {}
            CliOpt::Port(p) => {}
            CliOpt::Protocol(d) => {}
            CliOpt::Verbosity(l) => log.set_log_level(l.clone()),
        }
    }
    log.info(format!("running with options: {:?}", opts).as_str());
}
