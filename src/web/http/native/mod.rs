pub mod default;
pub mod display;

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Copy, Clone)]
pub enum HttpMethod {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
    Patch,
}

#[derive(Debug, Copy, Clone)]
pub enum HttpVersion {
    Http11,
    Http20,
    Http30,
}

#[derive(Debug, Copy, Clone)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    RequestEntityTooLarge = 413,
    RequestURITooLong = 414,
    UnsupportedMediaType = 415,
    RequestedRangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
}

#[derive(Debug)]
pub struct HttpLine {
    pub method: HttpMethod,
    pub uri: PathBuf,
    pub version: HttpVersion,
}

#[derive(Debug)]
pub struct HttpResponseLine {
    pub version: HttpVersion,
    pub status: HttpStatus,
}

impl HttpResponseLine {
    pub fn new(version: HttpVersion, status: HttpStatus) -> HttpResponseLine {
        HttpResponseLine { version, status }
    }
}

#[derive(Debug)]
pub struct HttpHeader {
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct HttpBody {
    pub tokens: Vec<String>,
}

impl HttpBody {
    pub fn new(v: String) -> Self {
        HttpBody { tokens: vec![v] }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: PathBuf,
    pub version: HttpVersion,
    pub header: HttpHeader,
    pub body: HttpBody,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpStatus,
    pub header: HttpHeader,
    pub body: HttpBody,
}

impl HttpResponse {
    pub fn add_header(&mut self, key: String, value: String) {
        let _ = self.header.headers.insert(key, value);
    }
    pub fn content_type(&mut self, v: String) {
        self.header.headers.insert(String::from("Content-Type"), v);
    }
    pub fn content_length(&mut self, v: usize) {
        self.header
            .headers
            .insert(String::from("Content-Length"), format!("{}", v));
    }
}

impl From<HttpStatus> for HttpResponse {
    fn from(status: HttpStatus) -> Self {
        let mut resp = HttpResponse::default();
        resp.status = status;
        resp.body = HttpBody::from(format!("{}", status));
        let len: usize = resp.body.tokens.iter().map(|v| v.bytes().len()).sum();
        resp.content_type(String::from("text/html"));
        resp.content_length(len);
        resp
    }
}
