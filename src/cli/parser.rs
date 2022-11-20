use crate::{
    cli::{defaults, err::CliError, CliOpt},
    APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION,
};
use clap::{App, Arg};
use log::{error, warn};
use std::{net::IpAddr, path::PathBuf, result::Result, str::FromStr};

pub struct CliConfig {
    log_level: log::LevelFilter,
    https: bool,
    cli_opts: Vec<CliOpt>,
}

impl CliConfig {
    pub fn log_level(&self) -> log::LevelFilter {
        self.log_level
    }
    pub fn https(&self) -> bool {
        self.https
    }
    pub fn cli_opts(&self) -> Vec<CliOpt> {
        self.cli_opts.to_vec()
    }
}

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
                .value_name("IP")
                .max_values(1)
                .long_help("Sets the server IP (v4/v6) address"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .required(false)
                .takes_value(true)
                .value_name("PORT")
                .max_values(1)
                .long_help("Sets the server port number [possible values: 1..65535]"),
        )
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .required(false)
                .takes_value(true)
                .value_name("ROOT_PATH")
                .max_values(1)
                .long_help("Sets the server root/public_html/wwwroot directory"),
        )
        .arg(
            Arg::with_name("loglevel")
                .short("l")
                .long("loglevel")
                .required(false)
                .takes_value(true)
                .possible_values(&["error", "warn", "info", "debug", "trace"])
                .value_name("LOG_LEVEL")
                .max_values(1)
                .long_help("Sets the server logging verbosity"),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .required(false)
                .takes_value(true)
                .value_name("N_THREADS")
                .max_values(1)
                .long_help(
                    "Sets the number of threads used by the server [possible values: 1..10]",
                ),
        )
        .arg(
            Arg::with_name("https")
                .short("s")
                .long("https")
                .required(false)
                .requires_all(&["https-cert", "https-key"])
                .takes_value(false)
                .long_help("Use https, requires a certificate and private key"),
        )
        .arg(
            Arg::with_name("https-cert")
                .short("c")
                .long("https-cert")
                .requires("https")
                .takes_value(true)
                .value_name("CERT_PATH")
                .max_values(1)
                .long_help("Path to the server certificate file"),
        )
        .arg(
            Arg::with_name("https-key")
                .short("k")
                .long("https-key")
                .requires("https")
                .takes_value(true)
                .value_name("KEY_PATH")
                .max_values(1)
                .long_help("Path to the server private key file"),
        )
        .get_matches()
}

pub fn parse_matches(matches: &clap::ArgMatches) -> Result<CliConfig, CliError> {
    let https = if let CliOpt::Https(v) = https(matches)? {
        v
    } else {
        defaults::https()
    };

    let log_level = if let CliOpt::Verbosity(v) = loglevel(matches)? {
        v
    } else {
        defaults::loglevel()
    };

    let cli_opts = vec![
        address(matches)?,
        port(matches)?,
        directory(matches)?,
        threads(matches)?,
        https_cert(matches, &https)?,
        https_priv_key(matches, &https)?,
    ];

    Ok(CliConfig {
        log_level,
        https,
        cli_opts,
    })
}

fn address(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    if let Some(v) = matches.value_of("address") {
        match v.parse::<IpAddr>() {
            Ok(v) => Ok(CliOpt::Address(v)),
            Err(e) => {
                error!("failed to parse the specified address: {}", e);
                Err(CliError::Parse(e.to_string()))
            }
        }
    } else {
        warn!(
            "address not specified, using default: {:?}",
            defaults::address()
        );
        Ok(CliOpt::Address(defaults::address()))
    }
}

fn port(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    if let Some(v) = matches.value_of("port") {
        match v.parse::<u16>() {
            Ok(v) => Ok(CliOpt::Port(v)),
            Err(e) => {
                error!("failed to parse the specified port: {}", e);
                Err(CliError::Parse(e.to_string()))
            }
        }
    } else {
        warn!("port not specified, using default: {}", defaults::port());
        Ok(CliOpt::Port(defaults::port()))
    }
}

fn directory(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    if let Some(v) = matches.value_of("directory") {
        match PathBuf::from(v).canonicalize() {
            Ok(v) => Ok(CliOpt::Directory(v)),
            Err(e) => {
                error!("the specified path doesn't exist: {}", e);
                Err(CliError::Parse(e.to_string()))
            }
        }
    } else {
        warn!(
            "directory not specified, using default: {:?}",
            defaults::directory()
        );
        Ok(CliOpt::Directory(defaults::directory()))
    }
}

fn loglevel(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    if let Some(v) = matches.value_of("loglevel") {
        match log::LevelFilter::from_str(&v) {
            Ok(v) => Ok(CliOpt::Verbosity(v)),
            Err(e) => {
                error!("failed to parse log level: {}", e);
                Err(CliError::Parse(e.to_string()))
            }
        }
    } else {
        warn!(
            "log level not specified, using default: {}",
            defaults::loglevel()
        );
        Ok(CliOpt::Verbosity(defaults::loglevel()))
    }
}

fn threads(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    if let Some(v) = matches.value_of("threads") {
        match v.parse::<usize>() {
            Ok(v) => {
                if v > defaults::threads_max() {
                    warn!(
                        "max thread count is {}, using max. got: {}",
                        defaults::threads_max(),
                        v
                    );
                }
                Ok(CliOpt::Threads(std::cmp::min(v, defaults::threads_max())))
            }
            Err(e) => {
                error!("failed to parse thread count: {}", e);
                Err(CliError::Parse(e.to_string()))
            }
        }
    } else {
        warn!(
            "thread count not specified, using default: {}",
            defaults::threads()
        );
        Ok(CliOpt::Threads(defaults::threads()))
    }
}

fn https(matches: &clap::ArgMatches) -> Result<CliOpt, CliError> {
    if matches.is_present("https") {
        Ok(CliOpt::Https(true))
    } else {
        warn!(
            "https option not specified, using default: {}",
            defaults::https()
        );
        Ok(CliOpt::Https(defaults::https()))
    }
}

fn https_cert(matches: &clap::ArgMatches, https: &bool) -> Result<CliOpt, CliError> {
    if *https {
        if let Some(v) = matches.value_of("https-cert") {
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
                defaults::https_cert()
            );
            Ok(CliOpt::HttpsCert(Some(defaults::https_cert())))
        }
    } else {
        Ok(CliOpt::HttpsCert(None))
    }
}

fn https_priv_key(matches: &clap::ArgMatches, https: &bool) -> Result<CliOpt, CliError> {
    if *https {
        if let Some(v) = matches.value_of("https-key") {
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
                defaults::https_priv_key()
            );
            Ok(CliOpt::HttpsPrivKey(Some(defaults::https_priv_key())))
        }
    } else {
        Ok(CliOpt::HttpsPrivKey(None))
    }
}
