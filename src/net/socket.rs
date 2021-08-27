use crate::{cli::CliOpt, net::dproto::DataProtocol};
use std::{
    io::{Error, ErrorKind, Result, Write},
    net::{Incoming, IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket},
};

pub enum Socket {
    Tcp(TcpSock),
    Udp(UdpSock),
}

pub trait TcpSockRw {
    fn read(&self) -> Incoming<'_>;
    fn write(&self, stream: &mut TcpStream, buf: &[u8]) -> Result<usize>;
}

pub struct TcpSock {
    socket: TcpListener,
    _address: SocketAddr,
}

impl TcpSock {
    pub fn new(addr: IpAddr, port: u16) -> TcpSock {
        let _address = SocketAddr::new(addr, port);
        let socket = TcpListener::bind(_address).unwrap();

        TcpSock { socket, _address }
    }
}

impl TcpSockRw for TcpSock {
    fn read(&self) -> Incoming<'_> {
        self.socket.incoming()
    }
    fn write(&self, stream: &mut TcpStream, buf: &[u8]) -> Result<usize> {
        stream.write(buf)
    }
}

pub trait UdpSockRw {
    fn connect(&mut self, addr: SocketAddr) -> Result<()>;
    fn read(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>;
    fn write(&self, buf: &[u8]) -> Result<usize>;
}

pub struct UdpSock {
    socket: UdpSocket,
    _address: SocketAddr,
    _remote_address: Option<SocketAddr>,
}

impl UdpSock {
    pub fn new(addr: IpAddr, port: u16) -> UdpSock {
        let _address = SocketAddr::new(addr, port);
        let socket = UdpSocket::bind(_address).unwrap();

        UdpSock {
            socket,
            _address,
            _remote_address: None,
        }
    }
}

impl UdpSockRw for UdpSock {
    fn connect(&mut self, addr: SocketAddr) -> Result<()> {
        match self.socket.connect(addr) {
            Ok(_) => {
                self._remote_address = Some(addr);
                Ok(())
            }
            Err(e) => {
                self._remote_address = None;
                Err(e)
            }
        }
    }
    fn read(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.socket.recv_from(buf)
    }
    fn write(&self, buf: &[u8]) -> Result<usize> {
        match self._remote_address {
            Some(_) => self.socket.send(buf),
            None => Err(Error::new(
                ErrorKind::NotConnected,
                "this socket is not connected to a remote address",
            )),
        }
    }
}

pub struct SocketBuilder {
    addr: IpAddr,
    port: u16,
    proto: DataProtocol,
    other: Vec<CliOpt>,
}

impl SocketBuilder {
    pub fn new(opts: Vec<CliOpt>) -> SocketBuilder {
        let mut socket_builder = SocketBuilder::default();
        let opts_filtered = SocketBuilder::filter(opts);
        for opt in opts_filtered.0 {
            match opt {
                CliOpt::Address(a) => socket_builder.addr = a,
                CliOpt::Port(p) => socket_builder.port = p,
                CliOpt::Protocol(d) => socket_builder.proto = d,
                _ => {}
            }
        }

        socket_builder
    }
    pub fn socket(&self) -> Socket {
        if self.proto == DataProtocol::Tcp {
            Socket::Tcp(TcpSock::new(self.addr, self.port))
        } else {
            Socket::Udp(UdpSock::new(self.addr, self.port))
        }
    }
    pub fn other(&self) -> Vec<CliOpt> {
        self.other.to_vec()
    }
    fn filter(opts: Vec<CliOpt>) -> (Vec<CliOpt>, Vec<CliOpt>) {
        (
            opts.iter()
                .filter(|opt| {
                    matches!(
                        opt,
                        CliOpt::Address(_) | CliOpt::Port(_) | CliOpt::Protocol(_)
                    )
                })
                .cloned()
                .collect(),
            opts.iter()
                .filter(|opt| {
                    !matches!(
                        opt,
                        CliOpt::Address(_) | CliOpt::Port(_) | CliOpt::Protocol(_)
                    )
                })
                .cloned()
                .collect(),
        )
    }
}

impl Default for SocketBuilder {
    fn default() -> Self {
        SocketBuilder {
            addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            proto: DataProtocol::Tcp,
            other: Vec::new(),
        }
    }
}
