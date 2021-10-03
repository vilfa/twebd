use crate::{
    cli::{Build, CliOpt, Other},
    net::{SocketBuilder, TcpSocket},
    srv::{Connection, Server, ServerError, ServerRootBuilder, SERVER_QUEUE_SIZE, SERVER_TOKEN},
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{
        buffer_to_string, HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse,
        TlsConfigBuilder, ToBuf,
    },
};
use log::{debug, error, info, trace};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    sync::Arc,
};

pub struct HttpsServer {
    socket: TcpSocket,
    poll: mio::Poll,
    events: mio::Events,
    connections: HashMap<mio::Token, Connection>,
    next_conn_id: usize,
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

        let poll = match mio::Poll::new() {
            Ok(v) => v,
            Err(e) => {
                error!("error creating io poll instance: `{:?}`", e);
                panic!();
            }
        };

        poll.registry()
            .register(socket.socket_mut(), SERVER_TOKEN, mio::Interest::READABLE);

        let events = mio::Events::with_capacity(SERVER_QUEUE_SIZE);

        HttpsServer {
            socket,
            poll,
            events,
            connections: HashMap::new(),
            next_conn_id: 1,
            threads,
            root: Arc::new(root),
            tls_config: Arc::new(tls_config),
        }
    }
    fn listen(&mut self) {
        info!(
            "listening for https connections on socket: `{:?}`",
            &self.socket
        );

        loop {
            match self.poll.poll(&mut self.events, None) {
                Ok(_) => {
                    for event in self.events.iter() {
                        match event.token() {
                            SERVER_TOKEN => match self.accept(self.poll.registry()) {
                                Ok(_) => {}
                                Err(e) => error!("error accepting connection: {:?}", e),
                            },
                            _ => self.event(self.poll.registry(), event),
                        }
                    }
                }
                Err(e) => {
                    error!("socket polling error: `{:?}`", e);
                }
            }
        }
    }
    fn accept(&self, registry: &mio::Registry) -> Result<(), ServerError> {
        loop {
            match self.socket.accept() {
                Ok((sock, addr)) => {
                    debug!(
                        "accepting new connection on socket: from: `{:?}` `{:?}`",
                        &sock, &addr
                    );

                    let tls_conn = rustls::ServerConnection::new(self.tls_config.clone()).unwrap();
                    let token = mio::Token(self.next_conn_id);
                    self.next_conn_id += 1;

                    let mut conn = Connection::new(sock, token, tls_conn);
                    conn.register(registry);
                    self.connections.insert(token, conn);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
                Err(e) => {
                    error!("error accepting connection: `{:?}`", e);
                    return Err(ServerError::SessionIoError(e));
                }
            }
        }
    }
    fn event(&self, registry: &mio::Registry, event: &mio::event::Event) {
        let token = event.token();
        if self.connections.contains_key(&token) {
            // self.connections
            //     .get_mut(&token)
            //     .unwrap()
            //     .ready(registry, event);
            self.handle(event, self.connections.get_mut(&token).unwrap());

            if self.connections[&token].is_closed() {
                self.connections.remove(&token);
            }
        }
    }
    fn handle(&self, event: &mio::event::Event, conn: &mut Connection) {
        if event.is_readable() {}

        if event.is_writable() {}

        if conn.is_closing() {
            conn.shutdown(std::net::Shutdown::Both, self.poll.registry());
        } else {
            conn.reregister(self.poll.registry());
        }
    }
}

fn handle(conn: &mut rustls::ServerConnection, root: Arc<PathBuf>) -> Result<Vec<u8>, ServerError> {
    debug!("recieved tls server session: `{:?}`", &conn);
    let mut buf = match BufReader::new(conn.reader()).fill_buf() {
        Ok(v) => {
            debug!("read data from session: {} bytes", v.len());
            v.to_vec()
        }
        Err(e) => {
            error!("error reading data from session: `{:?}`", e);
            return Err(ServerError::SessionIoError(e));
        }
    };
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
