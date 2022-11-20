use crate::web::{HttpParseError, HttpResponseError, TlsConfigError};

#[derive(Debug)]
pub enum ConnectionError {
    TlsRead(std::io::Error),
    TlsProcess(rustls::Error),
    TlsWrite(std::io::Error),
    PlainRead(std::io::Error),
    PlainWrite(std::io::Error),
}

#[derive(Debug)]
pub enum ServerRootError {
    Unknown,
}

#[derive(Debug)]
pub enum ServerError {
    Request(HttpParseError),
    RequestIo(std::io::Error),
    Response(HttpResponseError),
    Security(TlsConfigError),
    RootPath(ServerRootError),
    SessionIo(std::io::Error),
    Connection(ConnectionError),
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        Self::RequestIo(e)
    }
}

impl From<HttpParseError> for ServerError {
    fn from(e: HttpParseError) -> Self {
        Self::Request(e)
    }
}

impl From<HttpResponseError> for ServerError {
    fn from(e: HttpResponseError) -> Self {
        Self::Response(e)
    }
}

impl From<TlsConfigError> for ServerError {
    fn from(e: TlsConfigError) -> Self {
        Self::Security(e)
    }
}

impl From<ServerRootError> for ServerError {
    fn from(e: ServerRootError) -> Self {
        Self::RootPath(e)
    }
}

impl From<ConnectionError> for ServerError {
    fn from(e: ConnectionError) -> Self {
        Self::Connection(e)
    }
}
