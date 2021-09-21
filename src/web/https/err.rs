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
