// use crate::{
//     cli::{Build, CliOpt, Other},
//     net::{SocketBuilder, TcpSocket},
//     srv::{Server, ServerError, ServerRootBuilder},
//     syn::{ThreadPool, ThreadPoolBuilder},
//     web::{
//         buffer_to_string, HandleRequest, HandleResponse, HttpHandler, HttpRequest, HttpResponse,
//         ToBuf,
//     },
// };
// use log::{debug, error, info, trace};
// use std::{
//     io::{BufRead, BufReader, Write},
//     net::TcpStream,
//     path::PathBuf,
//     sync::Arc,
// };

// pub struct HttpServer {
//     socket: TcpSocket,
//     threads: ThreadPool,
//     root: Arc<PathBuf>,
// }

// impl Server<Self, ServerError> for HttpServer {
//     fn new(opts: Vec<CliOpt>) -> Self {
//         info!("initializing http server with options: `{:?}`", &opts);

//         let socket_builder = SocketBuilder::new(opts);
//         let thread_pool_builder = ThreadPoolBuilder::new(socket_builder.other());
//         let server_root_builder = ServerRootBuilder::new(thread_pool_builder.other());

//         let socket = socket_builder.build().unwrap();
//         let threads = thread_pool_builder.build().unwrap();
//         let root = server_root_builder.build().unwrap();

//         HttpServer {
//             socket,
//             threads,
//             root: Arc::new(root),
//         }
//     }
//     fn listen(&mut self) {
//         info!(
//             "listening for http connections on socket: `{:?}",
//             &self.socket
//         );
//         for stream in self.socket.incoming() {
//             let mut stream = stream.unwrap();
//             let root = self.root.clone();
//             self.threads.execute(move || {
//                 info!("received tcp connection");
//                 match handle(&mut stream, root) {
//                     Ok(buf) => {
//                         let _ = stream.write(&buf);
//                     }
//                     Err(e) => {
//                         error!("error reading tcp stream: `{:?}`", e);
//                     }
//                 }
//             })
//         }
//     }
// }

// fn handle(stream: &mut TcpStream, root: Arc<PathBuf>) -> Result<Vec<u8>, ServerError> {
//     trace!("received server session: `{:?}`", &stream);
//     let mut buf = match BufReader::new(stream).fill_buf() {
//         Ok(v) => {
//             debug!("read data from session: {} bytes", v.len());
//             v.to_vec()
//         }
//         Err(e) => {
//             error!("error reading data from session: `{:?}`", e);
//             return Err(ServerError::SessionIoError(e));
//         }
//     };
//     let request = request(&mut buf)?;
//     let response = response(&request, &root);
//     Ok(response.to_buf())
// }

// fn request(buf: &mut [u8]) -> Result<HttpRequest, ServerError> {
//     trace!("received buffer: `{:?}`", &buf);
//     trace!("buffer as string: `{:?}`", buffer_to_string(&buf)?);
//     HttpHandler::<HttpRequest>::handle(buf).map_err(|e| ServerError::from(e))
// }

// fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse {
//     trace!("parsed request: `{:?}`", &req);
//     HttpHandler::<HttpResponse>::handle(req, root)
// }
