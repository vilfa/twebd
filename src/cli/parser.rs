use crate::{
    cli::CliOpt,
    log::{backlog::Backlog, LogLevel, LogRecord},
    net::DataProtocol,
    srv::server::Server,
    APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION,
};
use clap::{App, Arg};
use std::{
    io::{Error, ErrorKind, Result},
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
};

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_DESCRIPTION)
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
            Arg::with_name("directory")
                .short("f")
                .long("directory")
                .required(false)
                .takes_value(true)
                .value_name("DIRECTORY")
                .max_values(1)
                .help("Sets the server directory"),
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
                .takes_value(false)
                .help("Hides timestamps when logging"),
        )
        .arg(
            Arg::with_name("hide-loglevel")
                .long("hide-loglevel")
                .required(false)
                .takes_value(false)
                .help("Hides loglevel when logging"),
        )
        .arg(
            Arg::with_name("https")
                .long("https")
                .required(false)
                .requires_all(&["https-cert", "https-priv-key"])
                .takes_value(false)
                .help("Use https. Must also specify certificate and private key"),
        )
        .arg(
            Arg::with_name("https-cert")
                .long("https-cert")
                .requires("https")
                .takes_value(true)
                .help("File path to server certificate file"),
        )
        .arg(
            Arg::with_name("https-priv-key")
                .long("https-priv-key")
                .requires("https")
                .takes_value(true)
                .help("File path to the server key file"),
        )
        .get_matches()
}

pub fn parse_matches<'a>(matches: &clap::ArgMatches<'a>) -> Result<(Vec<CliOpt>, Vec<LogRecord>)> {
    let mut cli_parser = CliParser::new(matches);
    let cli_opts = vec![
        cli_parser.address()?,
        cli_parser.port()?,
        cli_parser.protocol()?,
        cli_parser.directory()?,
        cli_parser.loglevel()?,
        cli_parser.threads()?,
        cli_parser.hide_loglevel()?,
        cli_parser.hide_timestamp()?,
        cli_parser.https()?,
        cli_parser.https_cert()?,
        cli_parser.https_priv_key()?,
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
            matches: matches,
            backlog: Vec::new(),
        }
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
            self.backlog.push(LogRecord::new(
                LogLevel::Warning,
                format!("address not specified, using default: `127.0.0.1`"),
            ));
            Ok(CliOpt::Address(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
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
            self.backlog.push(LogRecord::new(
                LogLevel::Warning,
                format!("port not specified, using default: `8080`"),
            ));
            Ok(CliOpt::Port(8080))
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
            self.backlog.push(LogRecord::new(
                LogLevel::Warning,
                format!("data protocol not specified, using default: `tcp`"),
            ));
            Ok(CliOpt::Protocol(DataProtocol::Tcp))
        }
    }
    fn directory(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("directory") {
            if Path::new(v).exists() {
                let abs_path = PathBuf::from(v).canonicalize()?;
                Ok(CliOpt::Directory(abs_path))
            } else {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("the specified path doesn't exist: `{}`", v),
                ))
            }
        } else {
            self.backlog.push(LogRecord::new(
                LogLevel::Warning,
                format!("directory not specified, using default: `./public`"),
            ));
            Ok(CliOpt::Directory(PathBuf::from("./public")))
        }
    }
    fn loglevel(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("loglevel") {
            match v.parse::<u8>() {
                Ok(v) => {
                    if v > LogLevel::Debug as u8 {
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
                format!(
                    "log level not specified, using default: `{:?}`",
                    LogLevel::default()
                ),
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
                format!(
                    "thread count not specified, using default: `{}`",
                    Server::default_threads()
                ),
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
    fn https(&mut self) -> Result<CliOpt> {
        if self.matches.is_present("https") {
            Ok(CliOpt::Https(true))
        } else {
            self.backlog.push(LogRecord::new(
                LogLevel::Warning,
                format!("https option not specified, using http"),
            ));
            Ok(CliOpt::Https(false))
        }
    }
    fn https_cert(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("https-cert") {
            if Path::new(v).exists() {
                let abs_path = PathBuf::from(v).canonicalize()?;
                Ok(CliOpt::HttpsCert(abs_path))
            } else {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("the specified certificate path doesn't exist: `{}`", v),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected a certificate path"),
            ))
        }
    }
    fn https_priv_key(&mut self) -> Result<CliOpt> {
        if let Some(v) = self.matches.value_of("https-priv-key") {
            if Path::new(v).exists() {
                let abs_path = PathBuf::from(v).canonicalize()?;
                Ok(CliOpt::HttpsPrivKey(abs_path))
            } else {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("the specified private key path doesn't exist: `{}`", v),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected a private key path"),
            ))
        }
    }
}

impl Backlog for CliParser<'_> {
    fn backlog(&self) -> Vec<LogRecord> {
        self.backlog.to_vec()
    }
}
