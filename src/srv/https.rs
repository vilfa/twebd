use crate::{
    cli::{Build, CliOpt, Other},
    log::native::LogRecord,
    net::socket::{Socket, SocketBuilder},
    srv::{err::ServerError, root::ServerRootBuilder, Server},
    syn::thread::{ThreadPool, ThreadPoolBuilder},
    web::{
        http::{
            native::{HttpRequest, HttpResponse},
            HandleRequest, HandleResponse, HttpHandler,
        },
        https::tls::{TlsConfig, TlsConfigBuilder},
    },
};
use std::{net::TcpStream, path::PathBuf};

pub struct HttpsServer {
    socket: Socket,
    threads: ThreadPool,
    root: std::sync::Arc<PathBuf>,
    tls_config: std::sync::Arc<TlsConfig>,
    _backlog: Vec<LogRecord>,
}

impl Server<Self, ServerError> for HttpsServer {
    fn new(opts: Vec<CliOpt>) -> Self {
        let socket_builder = SocketBuilder::new(opts);
        let thread_pool_builder = ThreadPoolBuilder::new(socket_builder.other());
        let server_root_builder = ServerRootBuilder::new(thread_pool_builder.other());
        let tls_config_builder = TlsConfigBuilder::new(server_root_builder.other());

        let socket = socket_builder.build().unwrap();
        let threads = thread_pool_builder.build().unwrap();
        let root = server_root_builder.build().unwrap();
        let tls_config = tls_config_builder.build().unwrap();

        HttpsServer {
            socket,
            threads,
            root: std::sync::Arc::new(root),
            tls_config: std::sync::Arc::new(tls_config),
            _backlog: Vec::new(),
        }
    }
    fn listen(&self) {
        match self.socket {
            Socket::Tcp(socket) => {}
            Socket::Udp(socket) => {}
        }
    }
    fn handle(&self, conn: &mut TcpStream) -> Result<Vec<u8>, ServerError> {}
}

fn request(buf: &'static mut [u8]) -> Result<HttpRequest, ServerError> {
    HttpHandler::<HttpRequest>::handle(buf).map_err(|e| ServerError::from(e))
}

fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
    HttpHandler::<HttpResponse>::handle(req, root)
}
