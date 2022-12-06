use crate::srv::ConnectionError;
use log::trace;
use std::io::{Read, Write};

pub struct Connection {
    socket: mio::net::TcpStream,
    token: mio::Token,
    wbuf: Vec<u8>,
    closing: bool,
    closed: bool,
}

impl Connection {
    pub fn new(socket: mio::net::TcpStream, token: mio::Token) -> Self {
        Connection {
            socket,
            token,
            wbuf: Vec::new(),
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
        let interest = mio::Interest::READABLE | mio::Interest::WRITABLE;
        registry
            .register(&mut self.socket, self.token, interest)
            .unwrap();
    }
    pub fn reregister(&mut self, registry: &mio::Registry) {
        let interest = mio::Interest::READABLE | mio::Interest::WRITABLE;
        registry
            .reregister(&mut self.socket, self.token, interest)
            .unwrap();
    }
    fn deregister(&mut self, registry: &mio::Registry) {
        registry.deregister(&mut self.socket).unwrap();
    }
    pub fn read(&mut self) -> Result<Vec<u8>, ConnectionError> {
        let mut bufsiz: usize = 1024;
        let mut buffer: Vec<u8> = vec![];
        loop {
            let mut buf: Vec<u8> = vec![0; bufsiz];
            match self.socket.read(&mut buf) {
                Ok(0) => return Ok(buffer),
                Ok(size) => {
                    buffer.append(&mut buf);
                    if size == bufsiz {
                        bufsiz *= 2;
                    } else {
                        return Ok(buffer);
                    }
                }
                Err(e) => return Err(ConnectionError::PlainRead(e)),
            }
        }
    }
    pub fn write_b(&mut self, buf: Vec<u8>) -> std::io::Result<usize> {
        self.wbuf.write(&buf)
    }
    pub fn write(&mut self) -> Result<usize, ConnectionError> {
        match self.socket.write(&self.wbuf) {
            Ok(size) => {
                trace!("write plaintext to session: {} bytes", size);
                self.wbuf.clear();
                Ok(size)
            }
            Err(e) => Err(ConnectionError::PlainWrite(e)),
        }
    }
}

pub struct SecureConnection {
    socket: mio::net::TcpStream,
    token: mio::Token,
    tls_conn: rustls::ServerConnection,
    closing: bool,
    closed: bool,
}

impl SecureConnection {
    pub fn new(
        socket: mio::net::TcpStream,
        token: mio::Token,
        tls_conn: rustls::ServerConnection,
    ) -> Self {
        SecureConnection {
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
                trace!("read tls from socket: {} bytes", size);
                Ok(size)
            }
            Err(e) => Err(ConnectionError::TlsRead(e)),
        }
    }
    pub fn process_tls(&mut self) -> Result<rustls::IoState, ConnectionError> {
        match self.tls_conn.process_new_packets() {
            Ok(v) => {
                trace!("successfully processed new tls packets");
                Ok(v)
            }
            Err(e) => Err(ConnectionError::TlsProcess(e)),
        }
    }
    pub fn read_plain(&mut self, size: usize) -> Result<Vec<u8>, ConnectionError> {
        let mut buf: Vec<u8> = vec![0; size];
        match self.tls_conn.reader().read(&mut buf) {
            Ok(size) => {
                trace!("read plaintext from session: {} bytes", size);
                Ok(buf)
            }
            Err(e) => Err(ConnectionError::PlainRead(e)),
        }
    }
    pub fn write_plain(&mut self, buf: Vec<u8>) -> Result<usize, ConnectionError> {
        match self.tls_conn.writer().write(&buf) {
            Ok(size) => {
                trace!("write plaintext to session: {} bytes", size);
                Ok(size)
            }
            Err(e) => Err(ConnectionError::PlainWrite(e)),
        }
    }
    pub fn write_tls(&mut self) -> Result<usize, ConnectionError> {
        match self.tls_conn.write_tls(&mut self.socket) {
            Ok(size) => {
                trace!("write tls to socket: {} bytes", size);
                Ok(size)
            }
            Err(e) => Err(ConnectionError::TlsWrite(e)),
        }
    }
}
