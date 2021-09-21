use crate::{
    cli::{Build, CliOpt, Other},
    log::{backlog::Backlog, LogRecord},
    web::https::err::TlsConfigError,
};
use rustls::internal::pemfile;
use std::{path::PathBuf, result::Result};

pub struct TlsConfigBuilder {
    https_enabled: bool,
    cert_path: PathBuf,
    priv_key_path: PathBuf,
    _other: Vec<CliOpt>,
    _backlog: Vec<LogRecord>,
}

pub struct TlsConfig {
    pub server_config: rustls::ServerConfig,
}

impl Build<Self, TlsConfig, TlsConfigError> for TlsConfigBuilder {
    fn from(opts: Vec<CliOpt>) -> Self {
        let mut tls_config_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Https(v) => tls_config_builder.https_enabled = v,
                CliOpt::HttpsCert(v) => tls_config_builder.cert_path = v,
                CliOpt::HttpsPrivKey(v) => tls_config_builder.priv_key_path = v,
                cli_opt => tls_config_builder.add_other(cli_opt.to_owned()),
            }
        }
        tls_config_builder
    }
    fn build(&self) -> Result<TlsConfig, TlsConfigError> {
        let cert_chain = load_cert(&self.cert_path)?;
        let priv_key = load_priv_key(&self.priv_key_path)?;
        let mut server_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
        server_config.set_single_cert(cert_chain, priv_key)?;
        Ok(TlsConfig { server_config })
    }
}

impl Other for TlsConfigBuilder {
    fn add_other(&mut self, o: CliOpt) {
        self._other.push(o);
    }
    fn other(&self) -> Vec<CliOpt> {
        self._other.to_vec()
    }
}

impl Backlog for TlsConfigBuilder {
    fn add_backlog(&mut self, v: LogRecord) {
        self._backlog.push(v);
    }
    fn backlog(&self) -> Vec<LogRecord> {
        self._backlog.to_vec()
    }
}

impl Default for TlsConfigBuilder {
    fn default() -> Self {
        TlsConfigBuilder {
            https_enabled: false,
            cert_path: PathBuf::new(),
            priv_key_path: PathBuf::new(),
            _other: Vec::new(),
            _backlog: Vec::new(),
        }
    }
}

fn load_priv_key(path: &PathBuf) -> Result<rustls::PrivateKey, TlsConfigError> {
    let handle = std::fs::File::open(path)?;
    let mut buf_reader = std::io::BufReader::new(handle);
    match pemfile::pkcs8_private_keys(&mut buf_reader).map_err(|e| {
        TlsConfigError::PrivateKeyError(format!(
            "failed to load private key: `{:?}`: `{:?}`",
            path, e,
        ))
    }) {
        Ok(priv_keys) => {
            if priv_keys.len() == 1 {
                Ok(priv_keys[0].clone())
            } else {
                Err(TlsConfigError::PrivateKeyError(format!(
                    "expected exactly one private key, got: `{}`",
                    priv_keys.len()
                )))
            }
        }
        Err(e) => Err(e),
    }
}

fn load_cert(path: &PathBuf) -> Result<Vec<rustls::Certificate>, TlsConfigError> {
    let handle = std::fs::File::open(path)?;
    let mut buf_reader = std::io::BufReader::new(handle);

    pemfile::certs(&mut buf_reader).map_err(|e| {
        TlsConfigError::CertificateError(format!(
            "failed to load certificate or chain: `{:?}`: `{:?}`",
            path, e
        ))
    })
}