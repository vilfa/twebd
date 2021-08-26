use crate::{
    cli::CliOpt,
    net::socket::{Sock, TcpSock, UdpSock},
    syn::thread::ThreadPool,
};

pub struct Server {
    opts: Vec<CliOpt>,
    socket: Sock,
    pool: ThreadPool,
}

impl Server {
    pub fn new(opts: Vec<CliOpt>) -> Server {
        Server { opts }
    }
    pub fn max_threads() -> u8 {
        10
    }
    pub fn default_threads() -> u8 {
        4
    }
}
