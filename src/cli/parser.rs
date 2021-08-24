use super::CliOpt;
use crate::{
    log::logger::{Log, Logger},
    log::LogLevel,
    net::dproto::DataProtocol,
};
use std::net::IpAddr;

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    clap_app!(twebd =>
        (version: "0.1.0")
        (author: "Luka Vilfan <luka.vilfan@protonmail.com>")
        (about: "A simple and lightweight web server daemon")
        (@arg address: -a --address +takes_value "Sets the listener ip address")
        (@arg port: -p --port +takes_value "Sets the listener port")
        (@arg protocol: -d --protocol +takes_value "Sets the listener protocol (either `tcp` or `udp`)")
        (@arg loglevel: -l --loglevel +takes_value "Sets the logging verbosity (0=off, 1=error, 2=warn, 3=info)")
    )
    .get_matches()
}

pub fn parse_matches_required<'a>(log: &Logger, matches: &clap::ArgMatches<'a>) -> Vec<CliOpt> {
    let mut options = Vec::new();

    vec!["address", "port", "protocol"]
        .iter()
        .for_each(|e| match &e[..] {
            "address" => {
                if let Some(v) = matches.value_of(e) {
                    match v.parse::<IpAddr>() {
                        Ok(v) => options.push(CliOpt::Address(v)),
                        Err(e) => {
                            log.err(
                                format!("failed to parse the specified address: {}", e).as_str(),
                            );
                            std::process::exit(-1);
                        }
                    }
                } else {
                    log.err("expected an address, got none");
                    std::process::exit(-1);
                }
            }
            "port" => {
                if let Some(v) = matches.value_of(e) {
                    match v.parse::<u16>() {
                        Ok(v) => options.push(CliOpt::Port(v)),
                        Err(e) => {
                            log.err(format!("failed to parse the specified port: {}", e).as_str());
                            std::process::exit(-1);
                        }
                    }
                } else {
                    log.err("expected a port, got none");
                    std::process::exit(-1);
                }
            }
            "protocol" => {
                if let Some(v) = matches.value_of(e) {
                    match &v[..] {
                        "tcp" => options.push(CliOpt::Protocol(DataProtocol::Tcp)),
                        "udp" => options.push(CliOpt::Protocol(DataProtocol::Udp)),
                        d => log.err(format!("unknown data protocol: `{}`", d).as_str()),
                    }
                } else {
                    log.err(format!("expected a protocol").as_str());
                    std::process::exit(-1);
                }
            }
            _ => {}
        });

    options
}

pub fn parse_matches_optional<'a>(log: &Logger, matches: &clap::ArgMatches<'a>) -> Vec<CliOpt> {
    let mut options = Vec::new();

    vec!["loglevel"].iter().for_each(|e| match &e[..] {
        "loglevel" => {
            if let Some(v) = matches.value_of(e) {
                match v.parse::<u8>() {
                    Ok(v) if v <= LogLevel::Debug as u8 => {
                        options.push(CliOpt::Verbosity(LogLevel::from(v)));
                    }
                    Ok(_) => {
                        log.warn(format!("unknown log level, using default").as_str());
                        options.push(CliOpt::Verbosity(LogLevel::default()));
                    }
                    Err(e) => {
                        log.err(format!("failed to parse log level: `{}`", e).as_str());
                        std::process::exit(-1);
                    }
                }
            } else {
                log.warn(format!("log level not specified, using default").as_str());
            }
        }
        _ => {}
    });

    options
}
