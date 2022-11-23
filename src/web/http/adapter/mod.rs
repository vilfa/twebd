pub mod request;
pub mod response;

use crate::web;

pub struct HttpAdapter {}

pub trait HttpReceiver<V, E>
where
    V: Sized,
    E: Sized,
{
    fn receive(buf: &mut [u8]) -> Result<V, E>;
}

pub trait HttpResponder<V>
where
    V: Sized,
{
    fn respond(request: &web::HttpRequest, srv_root: &std::path::PathBuf) -> V;
}
