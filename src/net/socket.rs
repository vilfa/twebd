use crate::{
    cli::CliOpt,
    net::{tcp::TcpSocket, udp::UdpSocket, DataProtocol},
};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub enum Socket {
    Tcp(TcpSocket),
    Udp(UdpSocket),
}

pub struct SocketBuilder {
    address: IpAddr,
    port: u16,
    protocol: DataProtocol,
    other: Vec<CliOpt>,
}

impl SocketBuilder {
    pub fn new(opts: Vec<CliOpt>) -> SocketBuilder {
        let mut socket_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Address(v) => socket_builder.address = v,
                CliOpt::Port(v) => socket_builder.port = v,
                CliOpt::Protocol(v) => socket_builder.protocol = v,
                cli_opt => socket_builder.other.push(cli_opt.to_owned()),
            }
        }

        socket_builder
    }
    pub fn socket(&self) -> Socket {
        match self.protocol {
            DataProtocol::Tcp => Socket::Tcp(TcpSocket::new(self.address, self.port)),
            DataProtocol::Udp => Socket::Udp(UdpSocket::new(self.address, self.port)),
        }
    }
    pub fn other(&self) -> Vec<CliOpt> {
        self.other.to_vec()
    }
}

impl Default for SocketBuilder {
    fn default() -> Self {
        SocketBuilder {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            protocol: DataProtocol::Tcp,
            other: Vec::new(),
        }
    }
}
