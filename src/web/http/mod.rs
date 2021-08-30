pub mod request;
pub mod response;

use std::path::PathBuf;

pub struct HttpRequest {
    method: HttpMethod,
    uri: PathBuf,
    version: HttpVersion,
    header: HttpHeader,
    body: HttpBody,
}

pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

pub enum HttpVersion {
    Http11,
    Http20,
    Http30,
}

pub struct HttpReqLine {}

pub struct HttpHeader {}

pub struct HttpBody {}
