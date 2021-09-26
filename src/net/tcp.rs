use std::net::{Incoming, IpAddr, SocketAddr, TcpListener};

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
}
