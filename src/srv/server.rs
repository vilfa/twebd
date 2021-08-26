use crate::{
    cli::CliOpt,
    log::LogLevel,
    net::socket::{Socket, SocketBuilder},
    syn::thread::{ThreadPool, ThreadPoolBuilder},
};

pub struct Server {
    opts: Vec<CliOpt>,
    socket: Socket,
    pool: ThreadPool,
}

impl Server {
    pub fn new(opts: Vec<CliOpt>) -> Server {
        let sock_builder = SocketBuilder::new(opts.to_vec());
        let pool_builder = ThreadPoolBuilder::new(sock_builder.other());

        let socket = sock_builder.socket();
        let pool = pool_builder.thread_pool();
        pool.log(
            LogLevel::Info,
            format!("starting server with options: {:?}", opts),
        );

        Server { opts, socket, pool }
    }
    pub fn max_threads() -> usize {
        10
    }
    pub fn default_threads() -> usize {
        4
    }
}
