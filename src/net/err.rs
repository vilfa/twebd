use std::io;

#[derive(Debug)]
pub enum SocketError {
    SocketInUse,
    SocketNotConnected,
    InsufficientPrivileges,
    SocketIoError(io::Error),
    General(String),
}

impl From<io::Error> for SocketError {
    fn from(e: io::Error) -> Self {
        Self::SocketIoError(e)
    }
}
