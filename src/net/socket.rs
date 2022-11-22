use crate::{
    cli::{Builder, CliOpt},
    net::{SimpleTcpSocket, SocketError, TcpSocket, UdpSocket},
};
use log::trace;
use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr},
};

#[derive(Debug)]
pub enum Socket {
    Tcp(TcpSocket),
    Udp(UdpSocket),
}

#[derive(Debug)]
pub struct SocketBuilder<T> {
    address: IpAddr,
    port: u16,
    _other: Vec<CliOpt>,
    socket_type: PhantomData<T>,
}

impl Builder<Self, TcpSocket, SocketError> for SocketBuilder<TcpSocket> {
    fn new(opts: Vec<CliOpt>) -> Self {
        let mut socket_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Address(v) => socket_builder.address = v,
                CliOpt::Port(v) => socket_builder.port = v,
                cli_opt => socket_builder.add_other(cli_opt.to_owned()),
            }
        }

        trace!("constructed socket builder: {:?}", &socket_builder);

        socket_builder
    }
    fn build(&self) -> Result<TcpSocket, SocketError> {
        Ok(TcpSocket::new(self.address, self.port))
    }
    fn add_other(&mut self, o: CliOpt) {
        self._other.push(o);
    }
    fn other(&self) -> Vec<CliOpt> {
        self._other.to_vec()
    }
}

impl Builder<Self, SimpleTcpSocket, SocketError> for SocketBuilder<SimpleTcpSocket> {
    fn new(opts: Vec<CliOpt>) -> Self {
        let mut socket_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Address(v) => socket_builder.address = v,
                CliOpt::Port(v) => socket_builder.port = v,
                cli_opt => socket_builder.add_other(cli_opt.to_owned()),
            }
        }

        trace!("constructed socket builder: {:?}", &socket_builder);

        socket_builder
    }
    fn build(&self) -> Result<SimpleTcpSocket, SocketError> {
        Ok(SimpleTcpSocket::new(self.address, self.port))
    }
    fn add_other(&mut self, o: CliOpt) {
        self._other.push(o);
    }
    fn other(&self) -> Vec<CliOpt> {
        self._other.to_vec()
    }
}

impl Default for SocketBuilder<TcpSocket> {
    fn default() -> Self {
        SocketBuilder::<TcpSocket> {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            _other: Vec::new(),
            socket_type: PhantomData,
        }
    }
}

impl Default for SocketBuilder<SimpleTcpSocket> {
    fn default() -> Self {
        SocketBuilder::<SimpleTcpSocket> {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            _other: Vec::new(),
            socket_type: PhantomData,
        }
    }
}
