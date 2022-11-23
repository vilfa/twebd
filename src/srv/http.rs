use crate::{
    cli::{Builder, CliOpt},
    net::{SimpleTcpSocket, SocketBuilder},
    srv::{ConnectionHandler, Server, ServerError, ServerRootBuilder},
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{HttpAdapter, HttpReceiver, HttpRequest, HttpResponder, HttpResponse, ToBuffer},
};
use log::{debug, error, info};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::PathBuf,
    sync::Arc,
};

pub struct HttpServer {
    socket: SimpleTcpSocket,
    threads: ThreadPool,
    root: Arc<PathBuf>,
}

impl Server<Self, ServerError> for HttpServer {
    fn new(opts: Vec<CliOpt>) -> Self {
        info!("initializing http server: {:?}", &opts);

        let socket_builder = SocketBuilder::<SimpleTcpSocket>::new(opts);
        let thread_pool_builder = ThreadPoolBuilder::new(socket_builder.other());
        let server_root_builder = ServerRootBuilder::new(thread_pool_builder.other());

        let socket = socket_builder.build().unwrap();
        let threads = thread_pool_builder.build().unwrap();
        let root = server_root_builder.build().unwrap();

        HttpServer {
            socket,
            threads,
            root: Arc::new(root),
        }
    }
    fn request(buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
        debug!("received {} byte buffer", buf.len());
        HttpAdapter::receive(buf).map_err(|e| ServerError::from(e))
    }
    fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
        debug!("parsed request: {:?}", &req);
        HttpAdapter::respond(req, root)
    }
}

impl ConnectionHandler<ServerError> for HttpServer {
    fn listen(&mut self) {
        info!("listening for connections on socket {:?}", &self.socket);
        for stream in self.socket.incoming() {
            let mut stream = stream.unwrap();
            let root = self.root.clone();
            self.threads.execute(move || {
                debug!("received tcp connection");
                match Self::handle(&mut stream, root) {
                    Ok(buf) => {
                        let _ = stream.write(&buf);
                    }
                    Err(e) => {
                        error!("error while handling connection: {:?}", e);
                    }
                }
            })
        }
    }
    fn handle(stream: &mut TcpStream, root: Arc<PathBuf>) -> Result<Vec<u8>, ServerError> {
        let mut buf = match BufReader::new(stream).fill_buf() {
            Ok(v) => {
                debug!("read data from session: {} bytes", v.len());
                v.to_vec()
            }
            Err(e) => {
                error!("error reading data from session: {:?}", e);
                return Err(ServerError::SessionIo(e));
            }
        };
        let request = Self::request(&mut buf)?;
        let response = Self::response(&request, &root);
        Ok(response.to_buf())
    }
}
