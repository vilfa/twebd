use std::io;

#[derive(Debug)]
pub enum SocketError {
    InUse,
    NotConnected,
    InsufficientPrivileges,
    Io(io::Error),
}

impl From<io::Error> for SocketError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
