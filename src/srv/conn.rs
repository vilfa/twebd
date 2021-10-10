use crate::srv::ConnectionError;
use log::{debug, error, trace};
use std::io::{Read, Write};

pub struct Connection {
    socket: mio::net::TcpStream,
    token: mio::Token,
    tls_conn: rustls::ServerConnection,
    closing: bool,
    closed: bool,
}

impl Connection {
    pub fn new(
        socket: mio::net::TcpStream,
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
    pub fn shutdown(&mut self, how: std::net::Shutdown, registry: &mio::Registry) {
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
    pub fn read_tls(&mut self) -> Result<usize, ConnectionError> {
        match self.tls_conn.read_tls(&mut self.socket) {
            Ok(size) => {
                debug!("read tls from socket: {} bytes", size);
                Ok(size)
            }
            Err(e) => {
                error!("error reading from socket: `{:?}`", e);
                Err(ConnectionError::TlsReadError(e))
            }
        }
    }
    pub fn process_tls(&mut self) -> Result<rustls::IoState, ConnectionError> {
        match self.tls_conn.process_new_packets() {
            Ok(v) => {
                debug!("successfully processed new tls packets");
                trace!("tls packet iostate: `{:?}`", &v);
                Ok(v)
            }
            Err(e) => {
                error!("error processing new tls packets: `{:?}`", e);
                Err(ConnectionError::TlsProcessError(e))
            }
        }
    }
    pub fn read_plain(&mut self, size: usize) -> Result<Vec<u8>, ConnectionError> {
        let mut buf: Vec<u8> = vec![0; size];
        match self.tls_conn.reader().read(&mut buf) {
            Ok(size) => {
                debug!("read plaintext from session: {} bytes", size);
                Ok(buf)
            }
            Err(e) => {
                error!("error reading plaintext from session: `{:?}`", e);
                Err(ConnectionError::PlainReadError(e))
            }
        }
    }
    pub fn write_plain(&mut self, buf: Vec<u8>) -> Result<usize, ConnectionError> {
        match self.tls_conn.writer().write(&buf) {
            Ok(size) => {
                debug!("write plaintext to session: {} bytes", size);
                Ok(size)
            }
            Err(e) => {
                error!("error writing plaintext to session `{:?}`", e);
                Err(ConnectionError::PlainWriteError(e))
            }
        }
    }
    pub fn write_tls(&mut self) -> Result<usize, ConnectionError> {
        match self.tls_conn.write_tls(&mut self.socket) {
            Ok(size) => {
                debug!("write tls to socket: {} bytes", size);
                Ok(size)
            }
            Err(e) => {
                error!("error writing tls to socket: `{:?}`", e);
                Err(ConnectionError::TlsWriteError(e))
            }
        }
    }
}
