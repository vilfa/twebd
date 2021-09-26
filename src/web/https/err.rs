#[derive(Debug)]
pub enum TlsConfigError {
    CertificateError(String),
    PrivateKeyError(String),
    FileReaderError(String),
    TlsError(rustls::Error),
}

impl From<std::io::Error> for TlsConfigError {
    fn from(e: std::io::Error) -> Self {
        TlsConfigError::FileReaderError(format!("{:?}", e))
    }
}

impl From<rustls::Error> for TlsConfigError {
    fn from(e: rustls::Error) -> Self {
        TlsConfigError::TlsError(e)
    }
}
