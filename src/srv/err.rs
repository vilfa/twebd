use crate::web::{HttpParseError, HttpResponseError, TlsConfigError};

#[derive(Debug)]
pub enum ConnectionError {
    TlsReadError(std::io::Error),
    TlsProcessError(rustls::Error),
    TlsWriteError(std::io::Error),
    PlainReadError(std::io::Error),
    PlainWriteError(std::io::Error),
}

#[derive(Debug)]
pub enum ServerRootError {
    General,
}

#[derive(Debug)]
pub enum ServerError {
    RequestError(HttpParseError),
    RequestErrorGen(std::io::Error),
    ResponseError(HttpResponseError),
    SecurityError(TlsConfigError),
    RootPathError(ServerRootError),
    SessionIoError(std::io::Error),
    ConnError(ConnectionError),
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        Self::RequestErrorGen(e)
    }
}

impl From<HttpParseError> for ServerError {
    fn from(e: HttpParseError) -> Self {
        Self::RequestError(e)
    }
}

impl From<HttpResponseError> for ServerError {
    fn from(e: HttpResponseError) -> Self {
        Self::ResponseError(e)
    }
}

impl From<TlsConfigError> for ServerError {
    fn from(e: TlsConfigError) -> Self {
        Self::SecurityError(e)
    }
}

impl From<ServerRootError> for ServerError {
    fn from(e: ServerRootError) -> Self {
        Self::RootPathError(e)
    }
}

impl From<ConnectionError> for ServerError {
    fn from(e: ConnectionError) -> Self {
        Self::ConnError(e)
    }
}
