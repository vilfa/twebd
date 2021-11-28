use crate::{
    cli::{Build, CliOpt, Other},
    net::{SocketBuilder, TcpSocket},
    srv::{
        Connection, Server, ServerError, ServerRootBuilder, ServerSecure, SERVER_QUEUE_SIZE,
        SERVER_SOCKET_TOKEN,
    },
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{
        buffer_to_string, HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse,
        TlsConfigBuilder, ToBuf,
    },
};
use log::{debug, error, info, trace};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub struct HttpsServer {
    pub socket: TcpSocket,
    pub connections: HashMap<mio::Token, Connection>,
    pub poll: mio::Poll,
    pub root: PathBuf,
    pub next_conn_id: usize,
    pub threads: ThreadPool,
    pub tls_config: Arc<rustls::ServerConfig>,
}

impl Server<Self, ServerError> for HttpsServer {
    fn new(opts: Vec<CliOpt>) -> Self {
        info!("initializing https server with options: `{:?}`", &opts);

        let socket_builder = SocketBuilder::<TcpSocket>::new(opts);
        let thread_pool_builder = ThreadPoolBuilder::new(socket_builder.other());
        let server_root_builder = ServerRootBuilder::new(thread_pool_builder.other());
        let tls_config_builder = TlsConfigBuilder::new(server_root_builder.other());

        let mut socket = socket_builder.build().unwrap();
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

        match poll
            .registry()
            .register(&mut socket, SERVER_SOCKET_TOKEN, mio::Interest::READABLE)
        {
            Ok(_) => debug!(
                "registered readable interest for server socket: `{:?}`",
                &socket
            ),
            Err(e) => {
                error!(
                    "error registering readable interest for server socket: `{:?}`: `{:?}`",
                    &socket, e
                );
                panic!();
            }
        }

        let connections = HashMap::new();
        let next_conn_id = 1;

        HttpsServer {
            socket,
            connections,
            poll,
            root,
            next_conn_id,
            threads,
            tls_config: Arc::new(tls_config),
        }
    }
    fn listen(&mut self) {
        let mut events = mio::Events::with_capacity(SERVER_QUEUE_SIZE);

        loop {
            match self.poll.poll(&mut events, None) {
                Ok(_) => {
                    for event in events.iter() {
                        match event.token() {
                            SERVER_SOCKET_TOKEN => match self.accept() {
                                Err(e) => error!("error accepting connection: {:?}", e),
                                _ => {}
                            },
                            _ => match self.event(event) {
                                Err(e) => error!("error handling request: `{:?}`", e),
                                _ => {}
                            },
                        }
                    }
                }
                Err(e) => {
                    error!("socket polling error: `{:?}`", e);
                }
            }
        }
    }
}

impl ServerSecure<Self, ServerError> for HttpsServer {
    fn accept(&mut self) -> Result<(), ServerError> {
        loop {
            match self.socket.accept() {
                Ok((socket, address)) => {
                    debug!(
                        "accepting new connection on socket: from: `{:?}` `{:?}`",
                        &socket, &address
                    );
                    let token = mio::Token(self.next_conn_id);
                    let tls_connection =
                        rustls::ServerConnection::new(self.tls_config.clone()).unwrap();
                    let mut connection = Connection::new(socket, token, tls_connection);
                    connection.register(self.poll.registry());
                    self.connections.insert(token, connection);

                    self.next_conn_id += 1;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
                Err(e) => {
                    error!("error accepting connection: `{:?}`", e);
                    return Err(ServerError::SessionIoError(e));
                }
            }
        }
    }
    fn event(&mut self, event: &mio::event::Event) -> Result<(), ServerError> {
        let token = event.token();
        if self.connections.contains_key(&token) {
            match handle(
                event,
                self.connections.get_mut(&token).unwrap(),
                &self.poll,
                &self.root,
            ) {
                Err(e) => error!("error handling connection: `{:?}`", e),
                _ => {}
            }

            if self.connections.get(&token).unwrap().is_closed() {
                self.connections.remove(&token);
            }
        }
        Ok(())
    }
}

fn handle(
    event: &mio::event::Event,
    conn: &mut Connection,
    poll: &mio::Poll,
    root: &PathBuf,
) -> Result<(), ServerError> {
    if event.is_readable() {
        conn.read_tls()?;
        if let Ok(io_state) = conn.process_tls() {
            if io_state.plaintext_bytes_to_read() > 0 {
                let mut buf = conn.read_plain(io_state.plaintext_bytes_to_read())?;
                let request = request(&mut buf)?;
                let response = response(&request, &root);
                conn.write_plain(response.to_buf())?;
            }
        }
    }

    if event.is_writable() {
        conn.write_tls()?;
    }

    if conn.is_closing() {
        conn.shutdown(std::net::Shutdown::Both, poll.registry());
    } else {
        conn.reregister(poll.registry());
    }

    Ok(())
}

fn request(buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
    trace!(
        "received buffer: as string: `{:?}` `{:?}`",
        buf,
        buffer_to_string(buf)?
    );
    HttpHandler::<HttpRequest>::handle(buf).map_err(|e| ServerError::from(e))
}

fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
    trace!("parsed request: `{:?}`", req);
    HttpHandler::<HttpResponse>::handle(req, root)
}
