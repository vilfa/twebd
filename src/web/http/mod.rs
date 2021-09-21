pub mod consts;
pub mod err;
pub mod interop;
pub mod native;
pub mod request;
pub mod response;

use crate::web::http;

pub struct HttpHandler;

pub trait HandleRequest<T, E: Sized> {
    fn handle(buf: &'static mut [u8]) -> Result<T, E>;
}

pub trait HandleResponse<T: Sized> {
    fn handle(request: &http::native::HttpRequest, srv_root: &std::path::PathBuf) -> T;
}
