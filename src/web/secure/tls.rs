use crate::{
    cli::CliOpt,
    log::{backlog::Backlog, LogRecord},
};
use rustls::internal::pemfile;
use std::{path::PathBuf, result::Result};

pub struct TlsConfigBuilder {
    backlog: Vec<LogRecord>,
    https_enabled: bool,
    cert_path: PathBuf,
    priv_key_path: PathBuf,
    other: Vec<CliOpt>,
}

impl TlsConfigBuilder {
    pub fn new(opts: Vec<CliOpt>) -> TlsConfigBuilder {
        let mut tls_config_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Https(v) => tls_config_builder.https_enabled = v,
                CliOpt::HttpsCert(v) => tls_config_builder.cert_path = v,
                CliOpt::HttpsPrivKey(v) => tls_config_builder.priv_key_path = v,
                cli_opt => tls_config_builder.other.push(cli_opt.to_owned()),
            }
        }

        tls_config_builder
    }
    pub fn tls_config(&self) -> Result<Option<rustls::ServerConfig>, TlsConfigError> {
        if self.https_enabled {
            let cert_chain = self.load_cert()?;
            let priv_key = self.load_priv_key()?;
            let mut tls_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
            tls_config.set_single_cert(cert_chain, priv_key)?;
            Ok(Some(tls_config))
        } else {
            Ok(None)
        }
    }
    pub fn other(&self) -> Vec<CliOpt> {
        self.other.to_vec()
    }
    fn load_cert(&self) -> Result<Vec<rustls::Certificate>, TlsConfigError> {
        let handle = std::fs::File::open(&self.cert_path)?;
        let mut buf_reader = std::io::BufReader::new(handle);

        pemfile::certs(&mut buf_reader).map_err(|e| {
            TlsConfigError::CertificateError(format!(
                "failed to load certificate (chain): `{:?}`: `{:?}`",
                &self.cert_path, e
            ))
        })
    }
    fn load_priv_key(&self) -> Result<rustls::PrivateKey, TlsConfigError> {
        let handle = std::fs::File::open(&self.priv_key_path)?;
        let mut buf_reader = std::io::BufReader::new(handle);
        match pemfile::rsa_private_keys(&mut buf_reader).map_err(|e| {
            TlsConfigError::PrivateKeyError(format!(
                "failed to load private key: `{:?}`: `{:?}`",
                &self.priv_key_path, e,
            ))
        }) {
            Ok(priv_keys) => {
                if priv_keys.len() == 1 {
                    Ok(priv_keys[0].clone())
                } else {
                    Err(TlsConfigError::PrivateKeyError(format!(
                        "expected a single private key, got: `{}`",
                        priv_keys.len()
                    )))
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl Backlog for TlsConfigBuilder {
    fn backlog(&self) -> Vec<LogRecord> {
        self.backlog.to_vec()
    }
}

impl Default for TlsConfigBuilder {
    fn default() -> Self {
        TlsConfigBuilder {
            backlog: Vec::new(),
            https_enabled: false,
            cert_path: PathBuf::new(),
            priv_key_path: PathBuf::new(),
            other: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum TlsConfigError {
    CertificateError(String),
    PrivateKeyError(String),
    FileReaderError(String),
    TlsError(rustls::TLSError),
}

impl From<std::io::Error> for TlsConfigError {
    fn from(e: std::io::Error) -> Self {
        TlsConfigError::FileReaderError(format!("{:?}", e))
    }
}

impl From<rustls::TLSError> for TlsConfigError {
    fn from(e: rustls::TLSError) -> Self {
        TlsConfigError::TlsError(e)
    }
}
