#[derive(Debug)]
pub enum TlsConfigError {
    Certificate(String),
    PrivateKey(String),
    UnresolvablePath(String),
    TlsError(rustls::Error),
}

impl From<std::io::Error> for TlsConfigError {
    fn from(e: std::io::Error) -> Self {
        TlsConfigError::UnresolvablePath(format!("{:?}", e))
    }
}

impl From<rustls::Error> for TlsConfigError {
    fn from(e: rustls::Error) -> Self {
        TlsConfigError::TlsError(e)
    }
}
