pub mod consts;
pub mod err;
pub mod interop;
pub mod native;
pub mod request;
pub mod response;

use crate::web::http;

pub struct HttpHandler<T>;

pub trait HandleRequest<V, E>
where
    V: Sized,
    E: Sized,
{
    fn handle(buf: &'static mut [u8]) -> Result<V, E>;
}

pub trait HandleResponse<V>
where
    V: Sized,
{
    fn handle(request: &http::native::HttpRequest, srv_root: &std::path::PathBuf) -> V;
}
