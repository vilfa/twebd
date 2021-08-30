use super::CliOpt;
use crate::{
    log::{LogLevel, LogRecord},
    net::dproto::DataProtocol,
    srv::server::Server,
};
use clap::{App, Arg};
use std::{
    io::{Error, ErrorKind, Result},
    net::IpAddr,
};

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    App::new("twebd")
        .version("0.1.0")
        .author("Luka Vilfan <luka.vilfan@protonmail.com>")
        .about("A simple server deamon")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .required(false)
                .takes_value(true)
                .value_name("ADDRESS")
                .max_values(1)
                .help("Sets the server ip address"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .required(false)
                .takes_value(true)
                .value_name("PORT")
                .max_values(1)
                .help("Sets the server port number [possible values: 1..65535]"),
        )
        .arg(
            Arg::with_name("protocol")
                .short("d")
                .long("protocol")
                .required(false)
                .takes_value(true)
                .possible_values(&["tcp", "udp"])
                .value_name("PROTOCOL")
                .max_values(1)
                .help("Sets the server data layer protocol"),
        )
        .arg(
            Arg::with_name("loglevel")
                .short("l")
                .long("loglevel")
                .required(false)
                .takes_value(true)
                .value_name("LEVEL")
                .max_values(1)
                .help("Sets the server logging verbosity [possible values: 0=Off..4=Debug]"),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .required(false)
                .takes_value(true)
                .value_name("N_THREADS")
                .max_values(1)
                .help("Sets the number of threads used by the server [possible values: 1..10]"),
        )
        .arg(
            Arg::with_name("hide-timestamp")
                .long("hide-timestamp")
                .required(false)
                .help("Hides timestamps when logging"),
        )
        .arg(
            Arg::with_name("hide-loglevel")
                .long("hide-loglevel")
                .required(false)
                .help("Hides loglevel when logging"),
        )
        .get_matches()
}

pub fn parse_matches<'a>(matches: &clap::ArgMatches<'a>) -> Result<(Vec<CliOpt>, Vec<LogRecord>)> {
    let mut cli_parser = CliParser::new(matches);
    let cli_opts = vec![
        cli_parser.address()?,
        cli_parser.port()?,
        cli_parser.protocol()?,
        cli_parser.loglevel()?,
        cli_parser.threads()?,
        cli_parser.hide_loglevel()?,
        cli_parser.hide_timestamp()?,
    ];

    Ok((cli_opts, cli_parser.backlog()))
}

struct CliParser<'a> {
    matches: &'a clap::ArgMatches<'a>,
    backlog: Vec<LogRecord>,
}

impl CliParser<'_> {
    fn new<'a>(matches: &'a clap::ArgMatches<'a>) -> CliParser<'a> {
        CliParser {
            matches,
            backlog: Vec::new(),
        }
    }
    fn backlog(&mut self) -> Vec<LogRecord> {
        self.backlog.to_vec()
    }
    fn address(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("address") {
            match v.parse::<IpAddr>() {
                Ok(v) => Ok(CliOpt::Address(v)),
                Err(e) => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("failed to parse the specified address: `{}`", e),
                )),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected an address, got none"),
            ))
        }
    }
    fn port(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("port") {
            match v.parse::<u16>() {
                Ok(v) => Ok(CliOpt::Port(v)),
                Err(e) => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("failed to parse the specified port: `{}`", e),
                )),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected a port, got none"),
            ))
        }
    }
    fn protocol(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("protocol") {
            match &v[..] {
                "tcp" => Ok(CliOpt::Protocol(DataProtocol::Tcp)),
                "udp" => Ok(CliOpt::Protocol(DataProtocol::Udp)),
                e => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unknown data protocol: `{}`", e),
                )),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected a protocol"),
            ))
        }
    }
    fn loglevel(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("loglevel") {
            match v.parse::<u8>() {
                Ok(v) => {
                    if v >= LogLevel::Debug as u8 {
                        self.backlog.push(LogRecord::new(
                            LogLevel::Warning,
                            format!("unknown log level, using default"),
                        ))
                    }
                    Ok(CliOpt::Verbosity(LogLevel::from(v)))
                }
                Err(e) => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("failed to parse log level: `{}`", e),
                )),
            }
        } else {
            self.backlog.push(LogRecord::new(
                LogLevel::Warning,
                format!("log level not specified, using default"),
            ));
            Ok(CliOpt::Verbosity(LogLevel::default()))
        }
    }
    fn threads(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("threads") {
            match v.parse::<usize>() {
                Ok(v) => {
                    if v > Server::max_threads() {
                        self.backlog.push(LogRecord::new(
                            LogLevel::Warning,
                            format!(
                                "max thread count is {}, using default. got: `{}`",
                                Server::max_threads(),
                                v
                            ),
                        ));
                    }
                    Ok(CliOpt::Threads(std::cmp::min(v, Server::max_threads())))
                }
                Err(e) => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("failed to parse thread count: `{}`", e),
                )),
            }
        } else {
            self.backlog.push(LogRecord::new(
                LogLevel::Warning,
                format!("thread count not specified, using default"),
            ));
            Ok(CliOpt::Threads(Server::default_threads()))
        }
    }
    fn hide_timestamp(&mut self) -> Result<CliOpt> {
        Ok(CliOpt::ShowTimestamp(
            !self.matches.is_present("hide-timestamp"),
        ))
    }
    fn hide_loglevel(&mut self) -> Result<CliOpt> {
        Ok(CliOpt::ShowLoglevel(
            !self.matches.is_present("hide-loglevel"),
        ))
    }
}
