use crate::{
    cli::{Build, CliOpt, Other},
    net::{Socket, SocketBuilder, TcpSocketIo},
    srv::{Server, ServerError, ServerRootBuilder},
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse, ToBuf},
};
use log::{debug, error, info, trace};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::PathBuf,
};

pub struct HttpServer {
    socket: Socket,
    threads: ThreadPool,
    root: std::sync::Arc<PathBuf>,
}

impl Server<Self, ServerError> for HttpServer {
    fn new(opts: Vec<CliOpt>) -> Self {
        info!("intializing http server with options: `{:?}`", &opts);

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
        }
    }
    fn listen(&self) {
        info!(
            "listening for http connections on socket: `{:?}",
            &self.socket
        );
        match &self.socket {
            Socket::Tcp(socket) => {
                for stream in socket.read() {
                    let root = self.root.clone();
                    self.threads.execute(move || {
                        info!("recieved tcp connection");
                        let mut stream = stream.unwrap();
                        match handle(&mut stream, root) {
                            Ok(buf) => {
                                let _ = stream.write(&buf);
                            }
                            Err(e) => {
                                error!("error reading tcp stream: `{:?}`", e);
                            }
                        }
                    })
                }
            }
            _ => {}
        }
    }
}

fn handle(data: &mut TcpStream, root: std::sync::Arc<PathBuf>) -> Result<Vec<u8>, ServerError> {
    debug!("recieved tcp stream: `{:?}`", &data);
    let mut reader = BufReader::new(data);
    let mut buf = reader.fill_buf()?.to_vec();
    let req = request(&mut buf)?;
    let resp = response(&req, &root);
    Ok(resp.to_buf())
}

fn request(buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
    trace!("recieved buffer: `{:?}`", &buf);
    HttpHandler::<HttpRequest>::handle(buf).map_err(|e| ServerError::from(e))
}

fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
    trace!("parsed request: `{:?}`", &req);
    HttpHandler::<HttpResponse>::handle(req, root)
}
