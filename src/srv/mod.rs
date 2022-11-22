pub mod conn;
pub mod err;
pub mod file;
pub mod http;
pub mod https;
pub mod log;
pub mod root;

use std::{net::TcpStream, path::PathBuf, sync::Arc};

pub use conn::Connection;
pub use err::{ConnectionError, ServerError, ServerRootError};
pub use file::{File, FileReader};
pub use http::HttpServer;
pub use https::HttpsServer;
pub use root::ServerRootBuilder;

use crate::{
    cli,
    web::{HttpRequest, HttpResponse},
};

pub const SERVER_SOCKET_TOKEN: mio::Token = mio::Token(0);
pub const SERVER_QUEUE_SIZE: usize = 256;

pub trait Server<T, E>
where
    T: Sized,
    E: Sized,
{
    fn new(opts: Vec<cli::CliOpt>) -> T;
    fn request(buf: &mut [u8]) -> Result<HttpRequest, E>;
    fn response(req: &HttpRequest, root: &PathBuf) -> HttpResponse;
}

pub trait ConnectionHandler<E>
where
    E: Sized,
{
    fn listen(&mut self);
    fn handle(stream: &mut TcpStream, root: Arc<PathBuf>) -> Result<Vec<u8>, E>;
}

pub trait SecureConnectionHandler<E>
where
    E: Sized,
{
    fn listen(&mut self);
    fn accept(&mut self) -> Result<(), E>;
    fn event(&mut self, event: &mio::event::Event) -> Result<(), E>;
    fn handle(
        event: &mio::event::Event,
        conn: &mut Connection,
        poll: &mio::Poll,
        root: &PathBuf,
    ) -> Result<(), E>;
}
