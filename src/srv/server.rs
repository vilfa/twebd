use crate::{
    cli::CliOpt,
    log::{LogLevel, LogRecord},
    net::socket::{Socket, SocketBuilder, TcpSockRw, UdpSockRw},
    syn::thread::{ThreadPool, ThreadPoolBuilder},
};

pub struct Server {
    _opts: Vec<CliOpt>,
    socket: Socket,
    pool: ThreadPool,
}

impl Server {
    pub fn new(opts: Vec<CliOpt>) -> Server {
        let sock_builder = SocketBuilder::new(&opts);
        let pool_builder = ThreadPoolBuilder::new(sock_builder.other());

        let socket = sock_builder.socket();
        let pool = pool_builder.thread_pool();

        pool.log(LogRecord::new(
            LogLevel::Info,
            format!(
                "initialized thread pool with: {} worker thread(s), {} log thread(s)",
                pool.size().0,
                pool.size().1
            ),
        ));
        pool.log(LogRecord::new(
            LogLevel::Info,
            format!("initialized server with options: {:?}", opts),
        ));

        Server {
            _opts: opts,
            socket,
            pool,
        }
    }
    pub fn listen(&self) {
        self.log(LogRecord::new(LogLevel::Info, format!("starting server")));
        match &self.socket {
            Socket::Tcp(socket) => {
                self.log(LogRecord::new(
                    LogLevel::Info,
                    format!("listening for connections on socket: `{:?}`", &socket),
                ));
                for stream in socket.read() {
                    self.log(LogRecord::new(
                        LogLevel::Info,
                        format!("tcp listener got a connection: `{:?}`", &stream),
                    ))
                }
            }
            Socket::Udp(socket) => {
                self.log(LogRecord::new(
                    LogLevel::Info,
                    format!("listening for connections on socket: `{:?}`", &socket),
                ));
                loop {
                    let mut buf: [u8; 512] = [0; 512];
                    match socket.read(&mut buf) {
                        Ok((bytes, addr)) => {
                            self.log(LogRecord::new(
                                LogLevel::Info,
                                format!("read {} bytes from socket address: `{:?}`", bytes, addr),
                            ));
                            self.log(LogRecord::new(
                                LogLevel::Debug,
                                format!("bytes recv: {:?}", &buf[0..bytes]),
                            ));
                            self.log(LogRecord::new(
                                LogLevel::Debug,
                                format!(
                                    "bytes recv as chars: {:?}",
                                    String::from_utf8_lossy(&buf[0..bytes])
                                ),
                            ));
                        }
                        Err(e) => self.log(LogRecord::new(
                            LogLevel::Error,
                            format!("couldn't read from udp socket: `{}`", e),
                        )),
                    }
                }
            }
        }
    }
    pub fn log(&self, record: LogRecord) {
        self.pool.log(record);
    }
    pub fn max_threads() -> usize {
        10
    }
    pub fn default_threads() -> usize {
        4
    }
}
