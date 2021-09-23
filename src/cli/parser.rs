use crate::{
    app::{APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION},
    cli::CliOpt,
    net::DataProtocol,
    srv,
};
use clap::{App, Arg};
use log::{error, warn};
use std::{
    io::{Error, ErrorKind, Result},
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
    str::FromStr,
};

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
                .possible_values(&["error", "warn", "info", "debug", "trace"])
                .value_name("LEVEL")
                .max_values(1)
                .help("Sets the server logging verbosity"),
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
        // .arg(
        //     Arg::with_name("hide-timestamp")
        //         .long("hide-timestamp")
        //         .required(false)
        //         .takes_value(false)
        //         .help("Hides timestamps when logging"),
        // )
        // .arg(
        //     Arg::with_name("hide-loglevel")
        //         .long("hide-loglevel")
        //         .required(false)
        //         .takes_value(false)
        //         .help("Hides loglevel when logging"),
        // )
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

pub fn parse_matches<'a>(matches: &clap::ArgMatches<'a>) -> Result<CliConfig> {
    let cli_opts = vec![
        address(matches)?,
        port(matches)?,
        protocol(matches)?,
        directory(matches)?,
        threads(matches)?,
        // hide_loglevel(matches)?,
        // hide_timestamp(matches)?,
        https_cert(matches)?,
        https_priv_key(matches)?,
    ];

    Ok(CliConfig {
        log_level: if let CliOpt::Verbosity(v) = loglevel(matches)? {
            v
        } else {
            log::LevelFilter::Info
        },
        https: if let CliOpt::Https(v) = https(matches)? {
            v
        } else {
            false
        },
        cli_opts,
    })
}

fn address(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("address") {
        match v.parse::<IpAddr>() {
            Ok(v) => Ok(CliOpt::Address(v)),
            Err(e) => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("failed to parse the specified address: `{}`", e),
            )),
        }
    } else {
        warn!("address not specified, using default: `127.0.0.1`");
        Ok(CliOpt::Address(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
    }
}

fn port(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("port") {
        match v.parse::<u16>() {
            Ok(v) => Ok(CliOpt::Port(v)),
            Err(e) => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("failed to parse the specified port: `{}`", e),
            )),
        }
    } else {
        warn!("port not specified, using default: `8080`");
        Ok(CliOpt::Port(8080))
    }
}

fn protocol(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("protocol") {
        match &v[..] {
            "tcp" => Ok(CliOpt::Protocol(DataProtocol::Tcp)),
            "udp" => Ok(CliOpt::Protocol(DataProtocol::Udp)),
            e => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("unknown data protocol: `{}`", e),
            )),
        }
    } else {
        warn!("data protocol not specified, using default: `tcp`");
        Ok(CliOpt::Protocol(DataProtocol::Tcp))
    }
}

fn directory(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("directory") {
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
        warn!("directory not specified, using default: `./public`");
        Ok(CliOpt::Directory(PathBuf::from("./public")))
    }
}

fn loglevel(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("loglevel") {
        match log::LevelFilter::from_str(&v) {
            Ok(v) => Ok(CliOpt::Verbosity(v)),
            Err(e) => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("failed to parse log level: `{}`", e),
            )),
        }
    } else {
        warn!(
            "log level not specified, using default: `{:?}`",
            log::LevelFilter::Info
        );
        Ok(CliOpt::Verbosity(log::LevelFilter::Info))
    }
}

fn threads(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("threads") {
        match v.parse::<usize>() {
            Ok(v) => {
                if v > srv::max_threads() {
                    warn!(
                        "max thread count is {}, using default. got: `{}`",
                        srv::max_threads(),
                        v
                    );
                }
                Ok(CliOpt::Threads(std::cmp::min(v, srv::max_threads())))
            }
            Err(e) => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("failed to parse thread count: `{}`", e),
            )),
        }
    } else {
        warn!(
            "thread count not specified, using default: `{}`",
            srv::default_threads()
        );
        Ok(CliOpt::Threads(srv::default_threads()))
    }
}

// fn hide_timestamp(matches: &clap::ArgMatches) -> Result<CliOpt> {
//     Ok(CliOpt::ShowTimestamp(
//         !matches.is_present("hide-timestamp"),
//     ))
// }

// fn hide_loglevel(matches: &clap::ArgMatches) -> Result<CliOpt> {
//     Ok(CliOpt::ShowLoglevel(
//         !matches.is_present("hide-loglevel"),
//     ))
// }

fn https(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if matches.is_present("https") {
        Ok(CliOpt::Https(true))
    } else {
        warn!("https option not specified, using http");
        Ok(CliOpt::Https(false))
    }
}

fn https_cert(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("https-cert") {
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

fn https_priv_key(matches: &clap::ArgMatches) -> Result<CliOpt> {
    if let Some(v) = matches.value_of("https-priv-key") {
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
