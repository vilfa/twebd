use crate::net::err::SocketError;
use std::{
    net::{IpAddr, SocketAddr},
    result::Result,
};

pub trait UdpSocketIo {
    fn connect(&mut self, addr: SocketAddr) -> Result<(), SocketError>;
    fn read(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), SocketError>;
    fn write(&self, buf: &[u8]) -> Result<usize, SocketError>;
}

#[derive(Debug)]
pub struct UdpSocket {
    socket: std::net::UdpSocket,
    remote_address: Option<SocketAddr>,
}

impl UdpSocket {
    pub fn new(addr: IpAddr, port: u16) -> UdpSocket {
        let address = SocketAddr::new(addr, port);
        let socket = std::net::UdpSocket::bind(address).unwrap();

        UdpSocket {
            socket,
            remote_address: None,
        }
    }
}

impl UdpSocketIo for UdpSocket {
    fn connect(&mut self, addr: SocketAddr) -> Result<(), SocketError> {
        match self.socket.connect(addr) {
            Ok(_) => {
                self.remote_address = Some(addr);
                Ok(())
            }
            Err(e) => {
                self.remote_address = None;
                Err(SocketError::SocketIoError(e))
            }
        }
    }
    fn read(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), SocketError> {
        self.socket
            .recv_from(buf)
            .or_else(|e| Err(SocketError::SocketIoError(e)))
    }
    fn write(&self, buf: &[u8]) -> Result<usize, SocketError> {
        match self.remote_address {
            Some(_) => self
                .socket
                .send(buf)
                .or_else(|e| Err(SocketError::SocketIoError(e))),
            None => Err(SocketError::SocketNotConnected),
        }
    }
}
