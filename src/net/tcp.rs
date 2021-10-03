use std::net::{Incoming, IpAddr, SocketAddr, TcpListener, TcpStream};

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
    pub fn incoming(&self) -> Incoming<'_> {
        self.socket.incoming()
    }
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr), std::io::Error> {
        self.socket.accept()
    }
    pub fn socket(&self) -> &TcpListener {
        &self.socket
    }
    pub fn socket_mut(&mut self) -> &mut TcpListener {
        &mut self.socket
    }
}
