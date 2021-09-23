pub mod err;
pub mod socket;
pub mod tcp;
pub mod udp;

pub use err::SocketError;
pub use socket::{Socket, SocketBuilder};
pub use tcp::{TcpSocket, TcpSocketIo};
pub use udp::{UdpSocket, UdpSocketIo};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataProtocol {
    Tcp,
    Udp,
}
