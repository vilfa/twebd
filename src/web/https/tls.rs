use crate::{
    cli::{Builder, CliOpt},
    web::TlsConfigError,
};
use log::{debug, trace};
use std::{path::PathBuf, result::Result};

#[derive(Debug)]
pub struct TlsConfigBuilder {
    https_enabled: bool,
    cert_path: PathBuf,
    priv_key_path: PathBuf,
    _other: Vec<CliOpt>,
}

impl Builder<Self, rustls::ServerConfig, TlsConfigError> for TlsConfigBuilder {
    fn new(opts: Vec<CliOpt>) -> Self {
        let mut tls_config_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Https(v) => tls_config_builder.https_enabled = v,
                CliOpt::HttpsCert(v) => tls_config_builder.cert_path = v.unwrap(),
                CliOpt::HttpsPrivKey(v) => tls_config_builder.priv_key_path = v.unwrap(),
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
    if !path.exists() {
        return Err(TlsConfigError::UnresolvablePath(
            path.to_str().unwrap().to_string(),
        ));
    }
    let handle = std::fs::File::open(path.canonicalize().unwrap())?;
    let mut buf_reader = std::io::BufReader::new(handle);
    match rustls_pemfile::pkcs8_private_keys(&mut buf_reader).map_err(|e| {
        TlsConfigError::PrivateKey(format!("failed to load private key: {:?}: {:?}", path, e,))
    }) {
        Ok(priv_keys) => {
            if priv_keys.len() == 1 {
                debug!("loaded private key");
                Ok(rustls::PrivateKey(priv_keys[0].to_vec()))
            } else {
                Err(TlsConfigError::PrivateKey(format!(
                    "expected exactly one private key, got: {}",
                    priv_keys.len()
                )))
            }
        }
        Err(e) => Err(e),
    }
}

fn load_cert(path: &PathBuf) -> Result<Vec<rustls::Certificate>, TlsConfigError> {
    if !path.exists() {
        return Err(TlsConfigError::UnresolvablePath(
            path.to_str().unwrap().to_string(),
        ));
    }
    let handle = std::fs::File::open(path.canonicalize().unwrap())?;
    let mut buf_reader = std::io::BufReader::new(handle);

    match rustls_pemfile::certs(&mut buf_reader) {
        Ok(v) => {
            debug!("loaded certificate(s)");
            Ok(v.iter()
                .map(|cert| rustls::Certificate(cert.to_vec()))
                .collect::<Vec<rustls::Certificate>>())
        }
        Err(e) => Err(TlsConfigError::Certificate(format!(
            "failed to load certificate or chain: {:?}: {:?}",
            path, e
        ))),
    }
}
