use super::{exit, CliOpt};
use crate::{
    log::logger::{Log, Logger},
    log::LogLevel,
    net::dproto::DataProtocol,
    srv::server::Server,
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
        (@arg threads: -t --threads +takes_value "Sets the server thread pool size (max 10)")
    )
    .get_matches()
}

pub fn parse_matches_required<'a>(matches: &clap::ArgMatches<'a>) -> Vec<CliOpt> {
    let logger = Logger::new();
    let mut options = Vec::new();

    vec!["address", "port", "protocol"]
        .iter()
        .for_each(|e| match &e[..] {
            "address" => {
                if let Some(v) = matches.value_of(e) {
                    match v.parse::<IpAddr>() {
                        Ok(v) => options.push(CliOpt::Address(v)),
                        Err(e) => {
                            logger.err(format!("failed to parse the specified address: {}", e));
                            exit(-1)
                        }
                    }
                } else {
                    logger.err(String::from("expected an address, got none"));
                    exit(-1)
                }
            }
            "port" => {
                if let Some(v) = matches.value_of(e) {
                    match v.parse::<u16>() {
                        Ok(v) => options.push(CliOpt::Port(v)),
                        Err(e) => {
                            logger.err(format!("failed to parse the specified port: {}", e));
                            exit(-1)
                        }
                    }
                } else {
                    logger.err(String::from("expected a port, got none"));
                    exit(-1)
                }
            }
            "protocol" => {
                if let Some(v) = matches.value_of(e) {
                    match &v[..] {
                        "tcp" => options.push(CliOpt::Protocol(DataProtocol::Tcp)),
                        "udp" => options.push(CliOpt::Protocol(DataProtocol::Udp)),
                        d => logger.err(format!("unknown data protocol: `{}`", d)),
                    }
                } else {
                    logger.err(format!("expected a protocol"));
                    exit(-1)
                }
            }
            _ => {}
        });

    options
}

pub fn parse_matches_optional<'a>(matches: &clap::ArgMatches<'a>) -> Vec<CliOpt> {
    let logger = Logger::new();
    let mut options = Vec::new();

    vec!["loglevel", "threads"]
        .iter()
        .for_each(|e| match &e[..] {
            "loglevel" => {
                if let Some(v) = matches.value_of(e) {
                    match v.parse::<u8>() {
                        Ok(v) if v <= LogLevel::Debug as u8 => {
                            options.push(CliOpt::Verbosity(LogLevel::from(v)));
                        }
                        Ok(_) => {
                            logger.warn(format!("unknown log level, using default"));
                            options.push(CliOpt::Verbosity(LogLevel::default()));
                        }
                        Err(e) => {
                            logger.err(format!("failed to parse log level: `{}`", e));
                            exit(-1)
                        }
                    }
                } else {
                    logger.warn(format!("log level not specified, using default"));
                    options.push(CliOpt::Verbosity(LogLevel::default()));
                }
            }
            "threads" => {
                if let Some(v) = matches.value_of(e) {
                    match v.parse::<u8>() {
                        Ok(v) if v <= 10 => {
                            options.push(CliOpt::Threads(v));
                        }
                        Ok(v) => {
                            logger.warn(format!(
                                "max thread count is 10, defaulting to 10. got: `{}`",
                                v
                            ));
                            options.push(CliOpt::Threads(Server::max_threads()));
                        }
                        Err(e) => {
                            logger.err(format!("failed to parse thread count: `{}`", e));
                            exit(-1);
                        }
                    }
                } else {
                    logger.warn(format!("thread count not specified, using default"));
                    options.push(CliOpt::Threads(Server::default_threads()));
                }
            }
            _ => {}
        });

    options
}
