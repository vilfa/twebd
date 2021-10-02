use crate::{
    cli::{Build, CliOpt, Other},
    net::{SocketBuilder, TcpSocket},
    srv::{Server, ServerError, ServerRootBuilder},
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{
        buffer_to_string, HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse,
        TlsConfigBuilder, ToBuf,
    },
};
use log::{debug, error, info, trace};
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    sync::Arc,
};

pub struct HttpsServer {
    socket: TcpSocket,
    threads: ThreadPool,
    root: Arc<PathBuf>,
    tls_config: Arc<rustls::ServerConfig>,
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
            root: Arc::new(root),
            tls_config: Arc::new(tls_config),
        }
    }
    fn listen(&self) {
        info!(
            "listening for https connections on socket: `{:?}`",
            &self.socket
        );
        for stream in self.socket.incoming() {
            let mut stream = stream.unwrap();
            let root = self.root.clone();
            let config = self.tls_config.clone();
            let mut conn = rustls::ServerConnection::new(config).unwrap();
            self.threads.execute(move || {
                match conn.read_tls(&mut stream) {
                    Ok(size) => debug!("read tls data from session: {} bytes", size),
                    Err(e) => error!("error reading tls data from session: `{:?}`", e),
                }
                match conn.process_new_packets() {
                    Ok(_) => debug!("successfully processed new tls packets"),
                    Err(e) => error!("error processing new tls packets: `{:?}`", e),
                }
                match handle(&mut conn, root) {
                    Ok(buf) => {
                        match conn.writer().write(&buf) {
                            Ok(size) => debug!("write data to session: {} bytes", size),
                            Err(e) => error!("error writing data to session: `{:?}`", e),
                        }
                        match conn.write_tls(&mut stream) {
                            Ok(size) => debug!("write tls data to session: {} bytes", size),
                            Err(e) => {
                                error!("error writing tls data to session: `{:?}`", e)
                            }
                        }
                    }
                    Err(e) => {
                        error!("error handling https connection: `{:?}`", e);
                    }
                }
            })
        }
    }
}

fn handle(conn: &mut rustls::ServerConnection, root: Arc<PathBuf>) -> Result<Vec<u8>, ServerError> {
    debug!("recieved tls server session: `{:?}`", &conn);
    let mut buf = Vec::new();
    match conn.reader().read_to_end(&mut buf) {
        Ok(size) => debug!("read data from session: {} bytes", size),
        Err(e) => {
            error!("error reading data from session: `{:?}`", e);
            return Err(ServerError::SessionIoError(e));
        }
    }
    // let mut buf = match BufReader::new(conn.reader()).fill_buf() {
    //     Ok(v) => {
    //         debug!("read data from session: {} bytes", v.len());
    //         v.to_vec()
    //     }
    //     Err(e) => {
    //         error!("error reading data from session: `{:?}`", e);
    //         return Err(ServerError::SessionIoError(e));
    //     }
    // };
    let request = request(&mut buf)?;
    let response = response(&request, &root);
    Ok(response.to_buf())
}

fn request(buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
    trace!("received buffer: `{:?}`", &buf);
    trace!("buffer as string: `{:?}`", buffer_to_string(&buf)?);
    HttpHandler::<HttpRequest>::handle(buf).map_err(|e| ServerError::from(e))
}

fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
    trace!("parsed request: `{:?}`", &req);
    HttpHandler::<HttpResponse>::handle(req, root)
}
