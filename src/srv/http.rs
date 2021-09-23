use crate::{
    cli::{Build, CliOpt, Other},
    log::LogRecord,
    net::{Socket, SocketBuilder, TcpSocketIo, UdpSocketIo},
    srv::{Server, ServerError, ServerRootBuilder},
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse, ToBuf},
};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::PathBuf,
};

pub struct HttpServer {
    socket: Socket,
    threads: ThreadPool,
    root: std::sync::Arc<PathBuf>,
    _backlog: Vec<LogRecord>,
}

impl Server<Self, ServerError> for HttpServer {
    fn new(opts: Vec<CliOpt>) -> Self {
        let socket_builder = SocketBuilder::new(opts);
        let thread_pool_builder = ThreadPoolBuilder::new(socket_builder.other());
        let server_root_builder = ServerRootBuilder::new(thread_pool_builder.other());

        let socket = socket_builder.build().unwrap();
        let threads = thread_pool_builder.build().unwrap();
        let root = server_root_builder.build().unwrap();

        HttpServer {
            socket,
            threads,
            root: std::sync::Arc::new(root),
            _backlog: Vec::new(),
        }
    }
    fn listen(&self) {
        match &self.socket {
            Socket::Tcp(socket) => {
                for stream in socket.read() {
                    let root = self.root.clone();
                    self.threads.execute(move || {
                        let mut stream = stream.unwrap();
                        match handle(&mut stream, root) {
                            Ok(buf) => {
                                let _ = stream.write(&buf);
                            }
                            Err(e) => {}
                        }
                    })
                }
            }
            Socket::Udp(socket) => loop {
                let mut buf: [u8; 512] = [0; 512];
                match socket.read(&mut buf) {
                    Ok((size, addr)) => {}
                    Err(e) => {}
                }
            },
        }
    }
}

fn handle(data: &mut TcpStream, root: std::sync::Arc<PathBuf>) -> Result<Vec<u8>, ServerError> {
    let mut reader = BufReader::new(data);
    let mut buf = reader.fill_buf()?.to_vec();
    let req = request(&mut buf)?;
    let resp = response(&req, &root);
    Ok(resp.to_buf())
}

fn request(buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
    HttpHandler::<HttpRequest>::handle(buf).map_err(|e| ServerError::from(e))
}

fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
    HttpHandler::<HttpResponse>::handle(req, root)
}
