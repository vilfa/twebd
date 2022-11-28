use crate::{
    cli::{default, err::CliError, CliOpt},
    APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION,
};
use clap::{value_parser, Arg, Command};
use log::{error, warn};
use std::{net::IpAddr, path::PathBuf, result::Result, str::FromStr};

pub struct CliConfig {
    cli_opts: Vec<CliOpt>,
}

impl CliConfig {
    pub fn log_level(&self) -> log::LevelFilter {
        for opt in &self.cli_opts {
            match opt {
                CliOpt::Verbosity(v) => return *v,
                _ => {}
            }
        }
        log::LevelFilter::Info
    }
    pub fn https(&self) -> bool {
        for opt in &self.cli_opts {
            match opt {
                CliOpt::Https(v) => return *v,
                _ => {}
            }
        }
        false
    }
    pub fn cli_opts(&self) -> Vec<CliOpt> {
        self.cli_opts.to_vec()
    }
}

pub fn parse_args() -> clap::ArgMatches {
    Command::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_DESCRIPTION)
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .required(false)
                .value_parser(value_parser!(IpAddr))
                .value_name("IP")
                .long_help("Sets the server IP (v4/v6) address"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .required(false)
                .value_parser(value_parser!(u16).range(1..65535))
                .value_name("PORT")
                .long_help("Sets the server port number"),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .required(false)
                .value_parser(value_parser!(std::path::PathBuf))
                .value_name("ROOT_PATH")
                .long_help("Sets the server root/public_html/wwwroot directory"),
        )
        .arg(
            Arg::new("loglevel")
                .short('l')
                .long("loglevel")
                .required(false)
                .value_parser(["error", "warn", "info", "debug", "trace"])
                .value_name("LOG_LEVEL")
                .long_help("Sets the server logging verbosity"),
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .required(false)
                .value_parser(value_parser!(u32).range(1..10))
                .value_name("N_THREADS")
                .long_help("Sets the number of worker threads used by the server"),
        )
        .arg(
            Arg::new("https")
                .short('s')
                .long("https")
                .required(false)
                .requires_all(&["https-cert", "https-key"])
                .value_parser(value_parser!(bool))
                .long_help("Use https, requires a certificate and private key"),
        )
        .arg(
            Arg::new("https-cert")
                .short('c')
                .long("https-cert")
                .requires("https")
                .value_parser(value_parser!(std::path::PathBuf))
                .value_name("CERT_PATH")
                .long_help("Path to the server certificate file"),
        )
        .arg(
            Arg::new("https-key")
                .short('k')
                .long("https-key")
                .requires("https")
                .value_parser(value_parser!(std::path::PathBuf))
                .value_name("KEY_PATH")
                .long_help("Path to the server private key file"),
        )
        .get_matches()
}

pub fn parse_matches(matches: &clap::ArgMatches) -> Result<CliConfig, CliError> {
    let cli_opts = vec![
        loglevel(matches)?,
        https(matches)?,
        address(matches)?,
        port(matches)?,
        directory(matches)?,
        threads(matches)?,
    ];

    let mut cli_config = CliConfig { cli_opts };

    let https = cli_config.https();
    cli_config.cli_opts.push(https_cert(matches, &https)?);
    cli_config.cli_opts.push(https_priv_key(matches, &https)?);

    Ok(cli_config)
}

fn address(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    match matches.try_get_one("address") {
        Ok(v) => match v {
            Some(v) => Ok(CliOpt::Address(*v)),
            None => {
                warn!(
                    "address not specified, using default: {:?}",
                    default::address()
                );
                Ok(CliOpt::Address(default::address()))
            }
        },
        Err(e) => {
            error!("failed to parse the specified address: {}", e);
            Err(CliError::Parse(e.to_string()))
        }
    }
}

fn port(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    match matches.try_get_one("port") {
        Ok(v) => match v {
            Some(v) => Ok(CliOpt::Port(*v)),
            None => {
                warn!("port not specified, using default: {}", default::port());
                Ok(CliOpt::Port(default::port()))
            }
        },
        Err(e) => {
            error!("failed to parse the specified port: {}", e);
            Err(CliError::Parse(e.to_string()))
        }
    }
}

fn directory(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    match matches.try_get_one("directory") {
        Ok(v) => match v { 
            Some(v) => Ok(CliOpt::Directory(*v)), 
            None => {
                warn!("directory not specified, using default: {:?}",default::directory());
                Ok(CliOpt::Directory(default::directory()))
            }},
        Err(e) => {
            error!("the specified path doesn't exist: {}", e);
            Err(CliError::Parse(e.to_string()))
        }
    }
}

fn loglevel(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    match matches.try_get_one("loglevel") {
            Ok(v) => match v {
                Some(v) => Ok(CliOpt::Verbosity(log::LevelFilter::from_str(&v).unwrap())),
                None => {warn!(
                    "log level not specified, using default: {}",
                    default::loglevel()
                );
                Ok(CliOpt::Verbosity(default::loglevel()))}
            }
            Err(e) => {
                error!("failed to parse log level: {}", e);
                Err(CliError::Parse(e.to_string()))
            }
    }
}

fn threads(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    match matches.try_get_one("threads") {
        match v.parse::<usize>() {
            Ok(v) => {
                if v > default::threads_max() {
                    warn!(
                        "max thread count is {}, using max. got: {}",
                        default::threads_max(),
                        v
                    );
                }
                Ok(CliOpt::Threads(std::cmp::min(v, default::threads_max())))
            }
            Err(e) => {
                error!("failed to parse thread count: {}", e);
                Err(CliError::Parse(e.to_string()))
            }
        }
    } else {
        warn!(
            "thread count not specified, using default: {}",
            default::threads()
        );
        Ok(CliOpt::Threads(default::threads()))
    }
}

fn https(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    if matches.is_present("https") {
        Ok(CliOpt::Https(true))
    } else {
        warn!(
            "https option not specified, using default: {}",
            default::https()
        );
        Ok(CliOpt::Https(default::https()))
    }
}

fn https_cert(matches: &clap::ArgMatches, https: &bool) -> Result<CliOpt, CliError> {
    if *https {
        match matches.try_get_one("https-cert") {
            match PathBuf::from(v).canonicalize() {
                Ok(v) => Ok(CliOpt::HttpsCert(Some(v))),
                Err(e) => {
                    error!("the specified certificate path doesn't exist: {}", e);
                    Err(CliError::Parse(e.to_string()))
                }
            }
        } else {
            warn!(
                "certificate path not specified, using default: {:?}",
                default::https_cert()
            );
            Ok(CliOpt::HttpsCert(Some(default::https_cert())))
        }
    } else {
        Ok(CliOpt::HttpsCert(None))
    }
}

fn https_priv_key(matches: &clap::ArgMatches, https: &bool) -> Result<CliOpt, CliError> {
    if *https {
        match matches.try_get_one("https-key") {
            match PathBuf::from(v).canonicalize() {
                Ok(v) => Ok(CliOpt::HttpsPrivKey(Some(v))),
                Err(e) => {
                    error!("the specified private key path doesn't exist: {}", e);
                    Err(CliError::Parse(e.to_string()))
                }
            }
        } else {
            warn!(
                "private key path not specified, using default: {:?}",
                default::https_priv_key()
            );
            Ok(CliOpt::HttpsPrivKey(Some(default::https_priv_key())))
        }
    } else {
        Ok(CliOpt::HttpsPrivKey(None))
    }
}
