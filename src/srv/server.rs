use crate::{
    cli::CliOpt,
    log::{backlog::Backlog, LogLevel, LogRecord},
    net::socket::{Socket, SocketBuilder, TcpSockRw, UdpSockRw},
    srv::file::ServerRootBuilder,
    syn::thread::{ThreadPool, ThreadPoolBuilder},
    web::http::{
        request::{HttpParseError, HttpRequest, HttpRequestParser},
        response::{HttpResponse, HttpResponseBuilder, HttpResponseError},
    },
};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::PathBuf,
};

pub struct Server {
    _opts: Vec<CliOpt>,
    socket: Socket,
    pool: ThreadPool,
    root: PathBuf,
}

impl Server {
    pub fn new(opts: Vec<CliOpt>) -> Server {
        let sock_builder = SocketBuilder::new(&opts);
        let pool_builder = ThreadPoolBuilder::new(sock_builder.other());
        let root_builder = ServerRootBuilder::new(pool_builder.other());

        let socket = sock_builder.socket();
        let pool = pool_builder.thread_pool();
        let root = root_builder.root();

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
            root,
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
                    ));
                    let mut stream = stream.unwrap();
                    match self.handle_conn(&mut stream) {
                        Ok(v) => {
                            let _ = stream.write(&v);
                            self.log(LogRecord::new(
                                LogLevel::Info,
                                format!("handle conn: sent response"),
                            ));
                            self.log(LogRecord::new(
                                LogLevel::Debug,
                                format!(
                                    "handle conn: sent response: `{:?}`",
                                    String::from_utf8_lossy(&v)
                                ),
                            ))
                        }
                        Err(e) => self.log(LogRecord::new(
                            LogLevel::Error,
                            format!("error handling conn: {:?}", e),
                        )),
                    }
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
                                format!("bytes recv: `{:?}`", &buf[0..bytes]),
                            ));
                            self.log(LogRecord::new(
                                LogLevel::Debug,
                                format!(
                                    "bytes recv as chars: `{:?}`",
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
    fn handle_conn(&self, stream: &mut TcpStream) -> Result<Vec<u8>, ServerError> {
        let mut reader = BufReader::new(stream);
        let mut buf = reader.fill_buf()?.to_vec();
        let request = self.parse_request(&mut buf)?;
        let response = self.build_response(&request)?;
        Ok(response.as_buf())
    }
    fn parse_request(&self, buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
        let request_parser = HttpRequestParser::new(buf)?;
        let (request, backlog) = (request_parser.request()?, request_parser.backlog());
        self.log(LogRecord::new(
            LogLevel::Debug,
            format!("handle conn: parsed request: {:#?}", &request),
        ));
        for rec in backlog {
            self.log(rec);
        }
        Ok(request)
    }
    fn build_response(&self, req: &HttpRequest) -> Result<HttpResponse, ServerError> {
        let response_builder = HttpResponseBuilder::new(req, &self.root);
        let (response, backlog) = (response_builder.response(), response_builder.backlog());
        self.log(LogRecord::new(
            LogLevel::Debug,
            format!("handle conn: built response: {:#?}", &response),
        ));
        for rec in backlog {
            self.log(rec)
        }
        Ok(response)
    }
}

#[derive(Debug)]
enum ServerError {
    RequestError(HttpParseError),
    RequestErrorGen(std::io::Error),
    ResponseError(HttpResponseError),
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        Self::RequestErrorGen(e)
    }
}

impl From<HttpParseError> for ServerError {
    fn from(e: HttpParseError) -> Self {
        Self::RequestError(e)
    }
}

impl From<HttpResponseError> for ServerError {
    fn from(e: HttpResponseError) -> Self {
        Self::ResponseError(e)
    }
}
