use crate::{cli::CliOpt, net::dproto::DataProtocol};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, UdpSocket};

pub enum Socket {
    Tcp(TcpSock),
    Udp(UdpSock),
}

pub trait SockRw {
    fn read(&self);
    fn write(&self);
}

pub struct TcpSock {
    socket: TcpListener,
    address: SocketAddr,
}

impl TcpSock {
    pub fn new(addr: IpAddr, port: u16) -> TcpSock {
        let address = SocketAddr::new(addr, port);
        let socket = TcpListener::bind(address).unwrap();

        TcpSock { socket, address }
    }
}

impl SockRw for TcpSock {
    fn read(&self) {}
    fn write(&self) {}
}

pub struct UdpSock {
    socket: UdpSocket,
    address: SocketAddr,
}

impl UdpSock {
    pub fn new(addr: IpAddr, port: u16) -> UdpSock {
        let address = SocketAddr::new(addr, port);
        let socket = UdpSocket::bind(address).unwrap();

        UdpSock { socket, address }
    }
}

impl SockRw for UdpSock {
    fn read(&self) {}
    fn write(&self) {}
}

pub struct SocketBuilder {
    addr: IpAddr,
    port: u16,
    proto: DataProtocol,
    other: Vec<CliOpt>,
}

impl SocketBuilder {
    fn new(opts: Vec<CliOpt>) -> SocketBuilder {
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
    fn socket(&self) -> Socket {
        if self.proto == DataProtocol::Tcp {
            Socket::Tcp(TcpSock::new(self.addr, self.port))
        } else {
            Socket::Udp(UdpSock::new(self.addr, self.port))
        }
    }
    fn other(&self) -> Vec<CliOpt> {
        self.other
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
