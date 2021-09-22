pub mod defaults;
pub mod err;
pub mod file;
pub mod http;
pub mod https;
pub mod root;
pub mod server;

use crate::cli;

// pub trait Server<T, V, E>
pub trait Server<T, E>
where
    T: Sized,
    // V: Sized + interop::ToBuf,
    E: Sized,
{
    fn new(opts: Vec<cli::CliOpt>) -> T;
    fn listen(&self);
    fn handle(&self, conn: &mut std::net::TcpStream) -> Result<Vec<u8>, E>;
}
