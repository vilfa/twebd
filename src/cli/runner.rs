use crate::cli::parser;
use crate::cli::CliOpt;
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
    opts.iter().for_each(|e| match e {
        CliOpt::Address(a) => {}
        CliOpt::Port(p) => {}
        CliOpt::Protocol(d) => {}
        CliOpt::Verbosity(l) => log.set_log_level(*l),
    });
    log.info(format!("running with options: {:?}", opts).as_str());
}
