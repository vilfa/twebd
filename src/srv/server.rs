// use crate::{
//     cli::CliOpt,
//     log::{
//         native::{LogLevel, LogRecord},
//         Backlog,
//     },
//     net::{
//         socket::{Socket, SocketBuilder},
//         tcp::TcpSocketIo,
//         udp::UdpSocketIo,
//     },
//     srv::err::ServerError,
//     syn::thread::{ThreadPool, ThreadPoolBuilder},
//     web::{
//         http::native::{HttpRequest, HttpResponse},
//         https::tls::TlsConfigBuilder,
//     },
// };
// use std::{
//     io::{BufRead, BufReader, Write},
//     net::TcpStream,
//     path::PathBuf,
//     sync::Arc,
// };

// pub struct Server {
//     socket: Socket,
//     thread_pool: ThreadPool,
//     server_root: PathBuf,
//     tls_config: Arc<rustls::ServerConfig>,
//     backlog: Vec<LogRecord>,
// }

// impl Server {
//     pub fn new(opts: Vec<CliOpt>) -> std::result::Result<Server, ServerError> {
//         let mut backlog = Vec::new();
//         backlog.push(logf!(
//             LogLevel::Info,
//             "initializing server with options: {:?}",
//             &opts
//         ));

//         let sock_builder = SocketBuilder::new(opts);
//         let pool_builder = ThreadPoolBuilder::new(sock_builder.other());
//         let root_builder = ServerRootBuilder::new(pool_builder.other());
//         let tls_builder = TlsConfigBuilder::new(root_builder.other());

//         let socket = sock_builder.socket();
//         let thread_pool = pool_builder.thread_pool();
//         let server_root = root_builder.root();

//         backlog.push(logf!(
//             LogLevel::Info,
//             "initialized thread pool with: {} worker thread(s), {} log thread(s)",
//             thread_pool.size().0,
//             thread_pool.size().1
//         ));

//         if tls_builder.https_enabled() {
//             let tls_config = tls_builder.tls_config()?;
//             Ok(Server {
//                 socket,
//                 tls_config,
//                 thread_pool,
//                 server_root,
//                 backlog,
//             })
//         } else {
//             Ok(Server {
//                 socket,
//                 tls_config: None,
//                 thread_pool,
//                 server_root,
//                 backlog,
//             })
//         }
//     }
//     pub fn listen(&self) {
//         self.log(logf!(LogLevel::Info, "starting server"));
//         match &self.socket {
//             Socket::Tcp(socket) => {
//                 if self.is_https() {
//                     // !!! TODO
//                     let server_session =
//                         rustls::ServerSession::new(&(self.tls_config.as_ref().unwrap()));
//                 } else {
//                 }
//                 self.log(logf!(
//                     LogLevel::Info,
//                     "listening for connections on socket: `{:?}`",
//                     &socket
//                 ));
//                 for stream in socket.read() {
//                     self.log(logf!(
//                         LogLevel::Info,
//                         "tcp listener got a connection: `{:?}`",
//                         &stream
//                     ));
//                     let mut stream = stream.unwrap();
//                     match self.handle_conn(&mut stream) {
//                         Ok(v) => {
//                             let _ = stream.write(&v);
//                             self.log(logf!(LogLevel::Info, "handle conn: sent response"));
//                             self.log(logf!(
//                                 LogLevel::Debug,
//                                 "handle conn: sent response: `{:?}`",
//                                 String::from_utf8_lossy(&v)
//                             ))
//                         }
//                         Err(e) => logf!(LogLevel::Error, "error handling conn: {:?}", e),
//                     }
//                 }
//             }
//             Socket::Udp(socket) => {
//                 self.log(logf!(
//                     LogLevel::Info,
//                     "listening for connections on socket: `{:?}`",
//                     &socket
//                 ));
//                 loop {
//                     let mut buf: [u8; 512] = [0; 512];
//                     match socket.read(&mut buf) {
//                         Ok((bytes, addr)) => {
//                             self.log(logf!(
//                                 LogLevel::Info,
//                                 "read {} bytes from socket address: `{:?}`",
//                                 bytes,
//                                 addr
//                             ));
//                             self.log(logf!(LogLevel::Debug, "bytes recv: `{:?}`", &buf[0..bytes]));
//                             self.log(logf!(
//                                 LogLevel::Debug,
//                                 "bytes recv as chars: `{:?}`",
//                                 String::from_utf8_lossy(&buf[0..bytes])
//                             ));
//                         }
//                         Err(e) => self.log(logf!(
//                             LogLevel::Error,
//                             "couldn't read from udp socket: `{}`",
//                             e
//                         )),
//                     }
//                 }
//             }
//         }
//     }
//     pub fn log(&self, record: LogRecord) {
//         self.thread_pool.log(record);
//     }
//     pub fn log_all(&self, records: &Vec<LogRecord>) {
//         for record in records {
//             self.thread_pool.log(record.to_owned());
//         }
//     }
//     fn is_https(&self) -> bool {
//         matches!(self.tls_config, Some(_))
//     }
//     fn handle_conn(&self, stream: &mut TcpStream) -> Result<Vec<u8>, ServerError> {
//         let mut reader = BufReader::new(stream);
//         let mut buf = reader.fill_buf()?.to_vec();
//         let request = self.parse_request(&mut buf)?;
//         let response = self.build_response(&request)?;
//         Ok(response.as_buf())
//     }
//     fn parse_request(&self, buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
//         let request_parser = HttpRequestParser::new(buf)?;
//         let (request, backlog) = (request_parser.request()?, request_parser.backlog());
//         self.log(logf!(
//             LogLevel::Debug,
//             "handle conn: parsed request: {:?}",
//             &request
//         ));
//         for rec in backlog {
//             self.log(rec);
//         }
//         Ok(request)
//     }
//     fn build_response(&self, req: &HttpRequest) -> Result<HttpResponse, ServerError> {
//         let response_builder = HttpResponseBuilder::new(req, &self.server_root);
//         let (response, backlog) = (response_builder.response(), response_builder.backlog());
//         self.log(logf!(
//             LogLevel::Debug,
//             "handle conn: built response: {:?}",
//             &response
//         ));
//         for rec in backlog {
//             self.log(rec)
//         }
//         Ok(response)
//     }
// }

// impl Backlog for Server {
//     fn backlog(&self) -> Vec<LogRecord> {
//         self.backlog.to_vec()
//     }
// }
