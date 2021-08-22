use crate::cli::parser;
use crate::cli::CliOpt;

pub fn run() {
    let matches = parser::parse_args();
    let opts: Vec<CliOpt> = vec![
        parser::parse_matches_required(&matches),
        parser::parse_matches_optional(&matches),
    ]
    .into_iter()
    .flatten()
    .collect();
    run_with_opts(opts);
}

fn run_with_opts(opts: Vec<CliOpt>) {
    println!("info: running with options: {:?}", opts);
}
