use crate::{
    cli::{Build, CliOpt, Other},
    net::{Socket, SocketBuilder, TcpSocketIo},
    srv::{Server, ServerError, ServerRootBuilder},
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{
        HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse, TlsConfig,
        TlsConfigBuilder, ToBuf,
    },
};
use log::{error, info};
use rustls::Session;
use std::{
    io::{Read, Write},
    path::PathBuf,
};

pub struct HttpsServer {
    socket: Socket,
    threads: ThreadPool,
    root: std::sync::Arc<PathBuf>,
    tls_config: TlsConfig,
}

impl Server<Self, ServerError> for HttpsServer {
    fn new(opts: Vec<CliOpt>) -> Self {
        info!("initializing https server with options: `{:?}`", &opts);

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
            tls_config,
        }
    }
    fn listen(&self) {
        info!(
            "listening for https connections on socket: `{:?}`",
            &self.socket
        );
        match &self.socket {
            Socket::Tcp(socket) => {
                for stream in socket.read() {
                    let config = std::sync::Arc::new(self.tls_config.server_config.clone());
                    let mut session = rustls::ServerSession::new(&config);
                    let root = self.root.clone();
                    self.threads.execute(move || {
                        let mut stream = stream.unwrap();
                        let _ = session.read_tls(&mut stream).unwrap();
                        let _ = session.process_new_packets();
                        match handle(&mut session, root) {
                            Ok(buf) => {
                                let _ = session.write(&buf);
                                let _ = session.write_tls(&mut stream);
                            }
                            Err(e) => {
                                error!("error handling https connection: `{:?}`", e);
                            }
                        }
                    })
                }
            }
            _ => {}
        }
    }
}

fn handle(
    session: &mut rustls::ServerSession,
    root: std::sync::Arc<PathBuf>,
) -> Result<Vec<u8>, ServerError> {
    let mut buf = Vec::new();
    let _ = session.read_to_end(&mut buf);
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
