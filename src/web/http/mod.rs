pub mod consts;
pub mod err;
pub mod interop;
pub mod native;
pub mod request;
pub mod response;

use crate::web;
use std::marker::PhantomData;

pub struct HttpHandler<T> {
    handler_type: PhantomData<T>,
}

impl<T> HttpHandler<T> {
    pub fn new() -> Self {
        HttpHandler {
            handler_type: PhantomData,
        }
    }
}

pub trait HandleRequest<V, E>
where
    V: Sized,
    E: Sized,
{
    fn handle(buf: &mut [u8]) -> Result<V, E>;
}

pub trait HandleResponse<V>
where
    V: Sized,
{
    fn handle(request: &web::HttpRequest, srv_root: &std::path::PathBuf) -> V;
}
