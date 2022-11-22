pub mod request;
pub mod response;

use crate::web;

use std::marker::PhantomData;

pub struct HttpAdapter<T> {
    handler_type: PhantomData<T>,
}

impl<T> HttpAdapter<T> {
    pub fn new() -> Self {
        HttpAdapter {
            handler_type: PhantomData::<T>,
        }
    }
}

pub trait HttpAcceptor<V, E>
where
    V: Sized,
    E: Sized,
{
    fn accept(buf: &mut [u8]) -> Result<V, E>;
}

pub trait HttpResponder<V>
where
    V: Sized,
{
    fn respond(request: &web::HttpRequest, srv_root: &std::path::PathBuf) -> V;
}
