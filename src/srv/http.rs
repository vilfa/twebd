use crate::{
    cli::{Builder, CliOpt},
    net::{SocketBuilder, TcpSocket},
    srv::{
        ConnectionHandler, Server, ServerError, ServerRootBuilder, SERVER_QUEUE_SIZE,
        SERVER_SOCKET_TOKEN,
    },
    syn::ThreadPoolBuilder,
    web::{HttpAdapter, HttpReceiver, HttpRequest, HttpResponder, HttpResponse, ToBuffer},
};
use log::{debug, error, info};
use std::{collections::HashMap, path::PathBuf};

use super::conn::Connection;

pub struct HttpServer {
    socket: TcpSocket,
    connections: HashMap<mio::Token, Connection>,
    poll: mio::Poll,
    root: PathBuf,
    nxid: usize,
}

impl Server<Self, ServerError> for HttpServer {
    fn new(opts: Vec<CliOpt>) -> Self {
        info!("initializing http server: {:?}", &opts);

        let socket_builder = SocketBuilder::<TcpSocket>::new(opts);
        let thread_pool_builder = ThreadPoolBuilder::new(socket_builder.other());
        let server_root_builder = ServerRootBuilder::new(thread_pool_builder.other());

        let mut socket = socket_builder.build().unwrap();
        let root = server_root_builder.build().unwrap();

        let poll = match mio::Poll::new() {
            Ok(v) => v,
            Err(e) => {
                error!("error creating poll instance: {:?}", e);
                panic!();
            }
        };

        match poll
            .registry()
            .register(&mut socket, SERVER_SOCKET_TOKEN, mio::Interest::READABLE)
        {
            Ok(_) => debug!(
                "registered readable interest for server socket: {:?}",
                &socket
            ),
            Err(e) => {
                error!(
                    "error registering readable interest for server socket: {:?}: {:?}",
                    &socket, e
                );
                panic!();
            }
        }

        let connections = HashMap::new();
        let nxid = 1;

        HttpServer {
            socket,
            connections,
            poll,
            root,
            nxid,
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
        info!(
            "listening for connections on socket {:?}",
            self.socket.socket().local_addr().unwrap()
        );
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
                                Err(e) => error!("error handling request: {:?}", e),
                                _ => {}
                            },
                        }
                    }
                }
                Err(e) => {
                    error!("socket polling error: {:?}", e);
                }
            }
        }
    }
    fn accept(&mut self) -> Result<(), ServerError> {
        loop {
            match self.socket.accept() {
                Ok((socket, _)) => {
                    debug!("accepting new connection on socket: from: {:?}", &socket);
                    let token = mio::Token(self.nxid);
                    let mut connection = Connection::new(socket, token);
                    connection.register(self.poll.registry());
                    self.connections.insert(token, connection);

                    self.nxid += 1;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
                Err(e) => {
                    error!("error accepting connection: {:?}", e);
                    return Err(ServerError::SessionIo(e));
                }
            }
        }
    }
    fn event(&mut self, event: &mio::event::Event) -> Result<(), ServerError> {
        let token = event.token();
        if self.connections.contains_key(&token) {
            match Self::handle(
                event,
                self.connections.get_mut(&token).unwrap(),
                &self.poll,
                &self.root,
            ) {
                Err(e) => error!("error handling connection: {:?}", e),
                _ => {}
            }

            if self.connections.get(&token).unwrap().is_closed() {
                self.connections.remove(&token);
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
            let mut buf = conn.read()?;
            let request = Self::request(&mut buf)?;
            let response = Self::response(&request, &root);
            conn.write_b(response.to_buf())?;
        }

        if event.is_writable() {
            conn.write()?;
        }

        if conn.is_closing() {
            conn.shutdown(std::net::Shutdown::Both, poll.registry());
        } else {
            conn.reregister(poll.registry());
        }

        Ok(())
    }
}
