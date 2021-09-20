pub mod err;
pub mod socket;
pub mod tcp;
pub mod udp;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataProtocol {
    Tcp,
    Udp,
}
