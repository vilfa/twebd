use log::{debug, error, trace};
use std::io::{Read, Write};

pub struct Connection {
    socket: std::net::TcpStream,
    token: mio::Token,
    tls_conn: rustls::ServerConnection,
    closing: bool,
    closed: bool,
}

impl Connection {
    pub fn new(
        socket: std::net::TcpStream,
        token: mio::Token,
        tls_conn: rustls::ServerConnection,
    ) -> Self {
        Connection {
            socket,
            token,
            tls_conn,
            closing: false,
            closed: false,
        }
    }
    pub fn shutdown(&self, how: std::net::Shutdown, registry: &mio::Registry) {
        match self.socket.shutdown(how) {
            _ => {
                self.closed = true;
                self.deregister(registry);
            }
        }
    }
    pub fn is_closing(&self) -> bool {
        self.closing
    }
    pub fn is_closed(&self) -> bool {
        self.closed
    }
    pub fn register(&mut self, registry: &mio::Registry) {
        let interest = self.interest();
        registry
            .register(&mut self.socket, self.token, interest)
            .unwrap();
    }
    pub fn reregister(&mut self, registry: &mio::Registry) {
        let interest = self.interest();
        registry
            .reregister(&mut self.socket, self.token, interest)
            .unwrap();
    }
    fn deregister(&mut self, registry: &mio::Registry) {
        registry.deregister(&mut self.socket).unwrap();
    }
    fn interest(&self) -> mio::Interest {
        let read = self.tls_conn.wants_read();
        let write = self.tls_conn.wants_write();
        if read && write {
            mio::Interest::READABLE | mio::Interest::WRITABLE
        } else if read {
            mio::Interest::READABLE
        } else {
            mio::Interest::WRITABLE
        }
    }
    fn read_tls(&mut self) {
        match self.tls_conn.read_tls(&mut self.socket) {
            Ok(size) => debug!("read tls from socket: {} bytes", size),
            Err(e) => error!("error reading from socket: `{:?}`", e),
        }
    }
    fn process_tls(&mut self) {
        match self.tls_conn.process_new_packets() {
            Ok(v) => {
                debug!("successfully processed new tls packets");
                trace!("tls packet iostate: `{:?}`", &v);
                if v.plaintext_bytes_to_read() > 0 {
                    self.read_plain(v.plaintext_bytes_to_read());
                }
            }
            Err(e) => error!("error processing new tls packets: `{:?}`", e),
        }
    }
    fn read_plain(&mut self, size: usize) {
        let mut buf: Vec<u8> = Vec::with_capacity(size);
        match self.tls_conn.reader().read(&mut buf) {
            Ok(v) => debug!("read plaintext from session: {} bytes", v),
            Err(e) => error!("error reading plaintext from session: `{:?}`", e),
        }
    }
    fn write_plain(&mut self, buf: Vec<u8>) {
        match self.tls_conn.writer().write(&buf) {
            Ok(size) => debug!("write plaintext to session: {} bytes", size),
            Err(e) => error!("error writing plaintext to session `{:?}`", e),
        }
    }
    fn write_tls(&mut self) {
        match self.tls_conn.write_tls(&mut self.socket) {
            Ok(size) => debug!("write tls to socket: {} bytes", size),
            Err(e) => error!("error writing tls to socket: `{:?}`", e),
        }
    }
}
