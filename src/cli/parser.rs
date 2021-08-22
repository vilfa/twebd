use crate::cli::CliOpt;
use crate::net::dproto::DataProtocol;
use std::net::IpAddr;

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    clap_app!(twebd =>
        (version: "0.1.0")
        (author: "Luka Vilfan <luka.vilfan@protonmail.com>")
        (about: "A simple and lightweight web server daemon")
        (@arg address: -a --address +takes_value * "Sets the listener ip address")
        (@arg port: -p --port +takes_value * "Sets the listener port")
        (@arg protocol: -d --protocol +takes_value * "Sets the listener protocol (either `tcp` or `udp`)")
        (@arg logging: -l --loglevel +takes_value "\nSets the logging verbosity\n(0=off, 1=error, 2=warn, 3=info)")
    )
    .get_matches()
}

pub fn parse_matches_required<'a>(matches: &clap::ArgMatches<'a>) -> Vec<CliOpt> {
    let mut options = Vec::new();

    vec!["address", "port", "protocol"]
        .iter()
        .for_each(|e| match &e[..] {
            "address" => {
                let v = matches
                    .value_of(e)
                    .expect("error: expected a listener address");

                match v.parse::<IpAddr>() {
                    Ok(v) => options.push(CliOpt::Address(v)),
                    Err(e) => panic!("error: failed to parse the specified address: {}", e),
                }
            }
            "port" => {
                let v = matches.value_of(e).expect("error: expected a port");

                match v.parse::<u16>() {
                    Ok(v) => options.push(CliOpt::Port(v)),
                    Err(e) => panic!("error: failed to parse the specified port: {}", e),
                }
            }
            "protocol" => {
                let v = matches.value_of(e).expect("error: expected a protocol");

                match &v[..] {
                    "tcp" => options.push(CliOpt::Protocol(DataProtocol::Tcp)),
                    "udp" => options.push(CliOpt::Protocol(DataProtocol::Udp)),
                    unk => panic!("error: unknown data protocol: `{}`", unk),
                }
            }
            _ => {}
        });

    options
}

pub fn parse_matches_optional<'a>(matches: &clap::ArgMatches<'a>) -> Vec<CliOpt> {
    let options = Vec::new();

    vec!["loglevel"].iter().for_each(|e| match &e[..] {
        "loglevel" => {}
        _ => {}
    });

    options
}
