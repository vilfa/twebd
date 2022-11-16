use crate::{
    cli::{Build, CliOpt, Other},
    web::TlsConfigError,
};
use log::trace;
use std::{path::PathBuf, result::Result};

#[derive(Debug)]
pub struct TlsConfigBuilder {
    https_enabled: bool,
    cert_path: PathBuf,
    priv_key_path: PathBuf,
    _other: Vec<CliOpt>,
}

pub struct TlsConfig {
    pub server_config: rustls::ServerConfig,
}

impl Build<Self, rustls::ServerConfig, TlsConfigError> for TlsConfigBuilder {
    fn new(opts: Vec<CliOpt>) -> Self {
        let mut tls_config_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Https(v) => tls_config_builder.https_enabled = v,
                CliOpt::HttpsCert(v) => tls_config_builder.cert_path = v,
                CliOpt::HttpsPrivKey(v) => tls_config_builder.priv_key_path = v,
                cli_opt => tls_config_builder.add_other(cli_opt.to_owned()),
            }
        }

        trace!("constructed tls config builder: `{:?}", &tls_config_builder);
        tls_config_builder
    }
    fn build(&self) -> Result<rustls::ServerConfig, TlsConfigError> {
        let cert_chain = load_cert(&self.cert_path)?;
        let priv_key = load_priv_key(&self.priv_key_path)?;
        let server_config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, priv_key)?;
        trace!("constructed server tls config");
        Ok(server_config)
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

impl Default for TlsConfigBuilder {
    fn default() -> Self {
        TlsConfigBuilder {
            https_enabled: false,
            cert_path: PathBuf::new(),
            priv_key_path: PathBuf::new(),
            _other: Vec::new(),
        }
    }
}

fn load_priv_key(path: &PathBuf) -> Result<rustls::PrivateKey, TlsConfigError> {
    trace!("loading private key");
    let handle = std::fs::File::open(path)?;
    let mut buf_reader = std::io::BufReader::new(handle);
    match rustls_pemfile::pkcs8_private_keys(&mut buf_reader).map_err(|e| {
        TlsConfigError::PrivateKeyError(format!("failed to load private key: {:?}: {:?}", path, e,))
    }) {
        Ok(priv_keys) => {
            if priv_keys.len() == 1 {
                Ok(rustls::PrivateKey(priv_keys[0].to_vec()))
            } else {
                Err(TlsConfigError::PrivateKeyError(format!(
                    "expected exactly one private key, got: {}",
                    priv_keys.len()
                )))
            }
        }
        Err(e) => Err(e),
    }
}

fn load_cert(path: &PathBuf) -> Result<Vec<rustls::Certificate>, TlsConfigError> {
    trace!("loading certificate");
    let handle = std::fs::File::open(path)?;
    let mut buf_reader = std::io::BufReader::new(handle);

    match rustls_pemfile::certs(&mut buf_reader) {
        Ok(v) => Ok(v
            .iter()
            .map(|cert| rustls::Certificate(cert.to_vec()))
            .collect::<Vec<rustls::Certificate>>()),
        Err(e) => Err(TlsConfigError::CertificateError(format!(
            "failed to load certificate or chain: {:?}: {:?}",
            path, e
        ))),
    }
}
