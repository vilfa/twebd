use crate::net::err::SocketError;
use std::{
    io::Write,
    net::{Incoming, IpAddr, SocketAddr, TcpListener, TcpStream},
    result::Result,
};

pub trait TcpSocketIo {
    fn read(&self) -> Incoming<'_>;
    fn write(&self, stream: &mut TcpStream, buf: &[u8]) -> Result<usize, SocketError>;
}

#[derive(Debug)]
pub struct TcpSocket {
    socket: TcpListener,
}

impl TcpSocket {
    pub fn new(addr: IpAddr, port: u16) -> TcpSocket {
        let address = SocketAddr::new(addr, port);
        let socket = TcpListener::bind(address).unwrap();

        TcpSocket { socket }
    }
}

impl TcpSocketIo for TcpSocket {
    fn read(&self) -> Incoming<'_> {
        self.socket.incoming()
    }
    fn write(&self, stream: &mut TcpStream, buf: &[u8]) -> Result<usize, SocketError> {
        stream
            .write(buf)
            .or_else(|e| Err(SocketError::SocketIoError(e)))
    }
}
