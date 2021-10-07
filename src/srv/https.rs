use crate::{
    cli::{Build, CliOpt, Other},
    net::{SocketBuilder, TcpSocket},
    srv::{
        Connection, Server, ServerError, ServerRootBuilder, SERVER_QUEUE_SIZE, SERVER_SOCKET_TOKEN,
    },
    syn::{ThreadPool, ThreadPoolBuilder},
    web::{
        buffer_to_string, HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse,
        TlsConfigBuilder, ToBuf,
    },
};
use log::{debug, error, info, trace};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, sync::Arc};

pub struct HttpsServer {
    socket: Arc<TcpSocket>,
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

        let events = mio::Events::with_capacity(SERVER_QUEUE_SIZE);

        HttpsServer {
            socket: Arc::new(socket),
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
                    for evt in self.events.iter() {
                        match evt.token() {
                            SERVER_SOCKET_TOKEN => match accept(
                                &self.socket,
                                &self.poll.registry(),
                                &mut self.connections,
                                self.tls_config.clone(),
                                self.next_conn_id,
                            ) {
                                Ok(_) => {
                                    self.next_conn_id += 1;
                                }
                                Err(e) => error!("error accepting connection: {:?}", e),
                            },
                            _ => match event(&evt, &mut self.connections, &self.poll, &self.root) {
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

fn accept(
    socket: &TcpSocket,
    registry: &mio::Registry,
    connections: &mut HashMap<mio::Token, Connection>,
    tls_config: Arc<rustls::ServerConfig>,
    next_conn_id: usize,
) -> Result<(), ServerError> {
    loop {
        match socket.accept() {
            Ok((sock, addr)) => {
                debug!(
                    "accepting new connection on socket: from: `{:?}` `{:?}`",
                    &sock, &addr
                );

                let tls_conn = rustls::ServerConnection::new(tls_config.clone()).unwrap();
                let token = mio::Token(next_conn_id);
                let mut conn = Connection::new(sock, token, tls_conn);
                conn.register(registry);
                connections.insert(token, conn);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
            Err(e) => {
                error!("error accepting connection: `{:?}`", e);
                return Err(ServerError::SessionIoError(e));
            }
        }
    }
}

fn event(
    event: &mio::event::Event,
    connections: &mut HashMap<mio::Token, Connection>,
    poll: &mio::Poll,
    root: &PathBuf,
) -> Result<(), ServerError> {
    let token = event.token();
    if connections.contains_key(&token) {
        handle(event, connections.get_mut(&token).unwrap(), poll, root)?;

        if connections[&token].is_closed() {
            connections.remove(&token);
        }
    }
    Ok(())
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
                let response = response(&request, root);
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
    trace!("received buffer: `{:?}`", buf);
    trace!("buffer as string: `{:?}`", buffer_to_string(buf)?);
    HttpHandler::<HttpRequest>::handle(buf).map_err(|e| ServerError::from(e))
}

fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
    trace!("parsed request: `{:?}`", req);
    HttpHandler::<HttpResponse>::handle(req, root)
}
