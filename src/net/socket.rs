use crate::{
    cli::{Build, CliOpt, Other},
    net::{DataProtocol, SocketError, TcpSocket, UdpSocket},
};
use log::trace;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub enum Socket {
    Tcp(TcpSocket),
    Udp(UdpSocket),
}

#[derive(Debug)]
pub struct SocketBuilder {
    address: IpAddr,
    port: u16,
    protocol: DataProtocol,
    _other: Vec<CliOpt>,
}

impl Build<Self, Socket, SocketError> for SocketBuilder {
    fn new(opts: Vec<CliOpt>) -> Self {
        let mut socket_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Address(v) => socket_builder.address = v,
                CliOpt::Port(v) => socket_builder.port = v,
                CliOpt::Protocol(v) => socket_builder.protocol = v,
                cli_opt => socket_builder.add_other(cli_opt.to_owned()),
            }
        }

        trace!("constructed socket builder: `{:?}`", &socket_builder);

        socket_builder
    }
    fn build(&self) -> Result<Socket, SocketError> {
        match self.protocol {
            DataProtocol::Tcp => Ok(Socket::Tcp(TcpSocket::new(self.address, self.port))),
            DataProtocol::Udp => Ok(Socket::Udp(UdpSocket::new(self.address, self.port))),
        }
    }
}

impl Other for SocketBuilder {
    fn add_other(&mut self, o: CliOpt) {
        self._other.push(o);
    }
    fn other(&self) -> Vec<CliOpt> {
        self._other.to_vec()
    }
}

impl Default for SocketBuilder {
    fn default() -> Self {
        SocketBuilder {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            protocol: DataProtocol::Tcp,
            _other: Vec::new(),
        }
    }
}
