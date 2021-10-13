pub mod conn;
pub mod err;
pub mod file;
pub mod http;
pub mod https;
pub mod root;

pub use conn::Connection;
pub use err::{ConnectionError, ServerError, ServerRootError};
pub use file::{File, FileReader};
pub use http::HttpServer;
pub use https::HttpsServer;
pub use root::ServerRootBuilder;

use crate::cli;

pub const SERVER_SOCKET_TOKEN: mio::Token = mio::Token(0);
pub const SERVER_QUEUE_SIZE: usize = 256;

pub trait Server<T, E>
where
    T: Sized,
    E: Sized,
{
    fn new(opts: Vec<cli::CliOpt>) -> T;
    fn listen(&mut self);
}

pub trait ServerSecure<T, E>
where
    T: Sized,
    E: Sized,
{
    fn accept(&mut self) -> Result<(), E>;
    fn event(&mut self, event: &mio::event::Event) -> Result<(), E>;
}
