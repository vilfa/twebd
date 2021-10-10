use std::{
    io,
    net::{IpAddr, SocketAddr},
};

#[derive(Debug)]
pub struct TcpSocket {
    socket: mio::net::TcpListener,
}

impl TcpSocket {
    pub fn new(addr: IpAddr, port: u16) -> TcpSocket {
        let address = SocketAddr::new(addr, port);
        let socket = mio::net::TcpListener::bind(address).unwrap();

        TcpSocket { socket }
    }
    pub fn accept(&self) -> Result<(mio::net::TcpStream, SocketAddr), std::io::Error> {
        self.socket.accept()
    }
    pub fn socket(&self) -> &mio::net::TcpListener {
        &self.socket
    }
    pub fn socket_mut(&mut self) -> &mut mio::net::TcpListener {
        &mut self.socket
    }
}

impl mio::event::Source for TcpSocket {
    fn register(
        &mut self,
        registry: &mio::Registry,
        token: mio::Token,
        interests: mio::Interest,
    ) -> io::Result<()> {
        self.socket.register(registry, token, interests)
    }
    fn reregister(
        &mut self,
        registry: &mio::Registry,
        token: mio::Token,
        interests: mio::Interest,
    ) -> io::Result<()> {
        self.socket.reregister(registry, token, interests)
    }
    fn deregister(&mut self, registry: &mio::Registry) -> io::Result<()> {
        self.socket.deregister(registry)
    }
}

#[derive(Debug)]
pub struct SimpleTcpSocket {
    socket: std::net::TcpListener,
}

impl SimpleTcpSocket {
    pub fn new(addr: IpAddr, port: u16) -> SimpleTcpSocket {
        let address = SocketAddr::new(addr, port);
        let socket = std::net::TcpListener::bind(address).unwrap();
        SimpleTcpSocket { socket }
    }
    pub fn incoming(&mut self) -> std::net::Incoming {
        self.socket.incoming()
    }
}
