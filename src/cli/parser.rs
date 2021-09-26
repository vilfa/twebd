use crate::{
    app::{APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION},
    cli::{defaults, CliOpt},
};
use clap::{App, Arg};
use log::{error, warn};
use std::{io::Result, net::IpAddr, path::PathBuf, str::FromStr};

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
        // .arg(
        //     Arg::with_name("protocol")
        //         .short("d")
        //         .long("protocol")
        //         .required(false)
        //         .takes_value(true)
        //         .possible_values(&["tcp", "udp"])
        //         .value_name("PROTOCOL")
        //         .max_values(1)
        //         .help("Sets the server data layer protocol"),
        // )
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

pub fn parse_matches(matches: &clap::ArgMatches) -> Result<CliConfig> {
    let cli_opts = vec![
        address(matches),
        port(matches),
        directory(matches),
        threads(matches),
        https_cert(matches),
        https_priv_key(matches),
    ];

    Ok(CliConfig {
        log_level: if let CliOpt::Verbosity(v) = loglevel(matches) {
            v
        } else {
            defaults::loglevel()
        },
        https: if let CliOpt::Https(v) = https(matches) {
            v
        } else {
            defaults::https()
        },
        cli_opts,
    })
}

fn address(matches: &clap::ArgMatches) -> CliOpt {
    if let Some(v) = matches.value_of("address") {
        match v.parse::<IpAddr>() {
            Ok(v) => CliOpt::Address(v),
            Err(e) => {
                error!(
                    "failed to parse the specified address: `{}`, using default: `{:?}`",
                    e,
                    defaults::address()
                );
                CliOpt::Address(defaults::address())
            }
        }
    } else {
        warn!(
            "address not specified, using default: `{:?}`",
            defaults::address()
        );
        CliOpt::Address(defaults::address())
    }
}

fn port(matches: &clap::ArgMatches) -> CliOpt {
    if let Some(v) = matches.value_of("port") {
        match v.parse::<u16>() {
            Ok(v) => CliOpt::Port(v),
            Err(e) => {
                error!(
                    "failed to parse the specified port: `{}`, using default: `{}`",
                    e,
                    defaults::port()
                );
                CliOpt::Port(defaults::port())
            }
        }
    } else {
        warn!("port not specified, using default: `{}`", defaults::port());
        CliOpt::Port(defaults::port())
    }
}

// fn protocol(matches: &clap::ArgMatches) -> CliOpt {
//     if let Some(v) = matches.value_of("protocol") {
//         match &v[..] {
//             "tcp" => CliOpt::Protocol(DataProtocol::Tcp),
//             "udp" => CliOpt::Protocol(DataProtocol::Udp),
//             e => {
//                 error!(
//                     "unknown data protocol: `{}`, using default: `{:?}`",
//                     e,
//                     defaults::protocol()
//                 );
//                 CliOpt::Protocol(defaults::protocol())
//             }
//         }
//     } else {
//         warn!(
//             "data protocol not specified, using default: `{:?}`",
//             defaults::protocol()
//         );
//         CliOpt::Protocol(defaults::protocol())
//     }
// }

fn directory(matches: &clap::ArgMatches) -> CliOpt {
    if let Some(v) = matches.value_of("directory") {
        match PathBuf::from(v).canonicalize() {
            Ok(v) => CliOpt::Directory(v),
            Err(e) => {
                error!(
                    "the specified path doesn't exist: `{}`, using default: `{:?}`",
                    e,
                    defaults::directory()
                );
                CliOpt::Directory(defaults::directory())
            }
        }
    } else {
        warn!(
            "directory not specified, using default: `{:?}`",
            defaults::directory()
        );
        CliOpt::Directory(defaults::directory())
    }
}

fn loglevel(matches: &clap::ArgMatches) -> CliOpt {
    if let Some(v) = matches.value_of("loglevel") {
        match log::LevelFilter::from_str(&v) {
            Ok(v) => CliOpt::Verbosity(v),
            Err(e) => {
                error!(
                    "failed to parse log level: `{}`, using default: `{}`",
                    e,
                    defaults::loglevel()
                );
                CliOpt::Verbosity(defaults::loglevel())
            }
        }
    } else {
        warn!(
            "log level not specified, using default: `{}`",
            defaults::loglevel()
        );
        CliOpt::Verbosity(defaults::loglevel())
    }
}

fn threads(matches: &clap::ArgMatches) -> CliOpt {
    if let Some(v) = matches.value_of("threads") {
        match v.parse::<usize>() {
            Ok(v) => {
                if v > defaults::threads_max() {
                    warn!(
                        "max thread count is {}, using max. got: `{}`",
                        defaults::threads_max(),
                        v
                    );
                }
                CliOpt::Threads(std::cmp::min(v, defaults::threads_max()))
            }
            Err(e) => {
                error!(
                    "failed to parse thread count: `{}`, using default: `{}`",
                    e,
                    defaults::threads()
                );
                CliOpt::Threads(defaults::threads())
            }
        }
    } else {
        warn!(
            "thread count not specified, using default: `{}`",
            defaults::threads()
        );
        CliOpt::Threads(defaults::threads())
    }
}

fn https(matches: &clap::ArgMatches) -> CliOpt {
    if matches.is_present("https") {
        CliOpt::Https(true)
    } else {
        warn!("https option not specified, using default");
        CliOpt::Https(defaults::https())
    }
}

fn https_cert(matches: &clap::ArgMatches) -> CliOpt {
    if let Some(v) = matches.value_of("directory") {
        match PathBuf::from(v).canonicalize() {
            Ok(v) => CliOpt::HttpsCert(v),
            Err(e) => {
                error!(
                    "the specified certificate path doesn't exist: `{}`, using default: `{:?}`",
                    e,
                    defaults::https_cert()
                );
                CliOpt::HttpsCert(defaults::https_cert())
            }
        }
    } else {
        warn!(
            "certificate path not specified, using default: `{:?}`",
            defaults::https_cert()
        );
        CliOpt::HttpsCert(defaults::https_cert())
    }
}

fn https_priv_key(matches: &clap::ArgMatches) -> CliOpt {
    if let Some(v) = matches.value_of("directory") {
        match PathBuf::from(v).canonicalize() {
            Ok(v) => CliOpt::HttpsPrivKey(v),
            Err(e) => {
                error!(
                    "the specified private key path doesn't exist: `{}`, using default: `{:?}`",
                    e,
                    defaults::https_priv_key()
                );
                CliOpt::HttpsPrivKey(defaults::https_priv_key())
            }
        }
    } else {
        warn!(
            "private key path not specified, using default: `{:?}`",
            defaults::https_priv_key()
        );
        CliOpt::HttpsPrivKey(defaults::https_priv_key())
    }
}
