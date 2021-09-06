pub mod request;
pub mod response;

use crate::web::http::request::HttpParseError;
use std::{collections::HashMap, path::PathBuf, result::Result};

pub const CRLF: &str = "\r\n"; // Line end sequence
pub const WSPC: &str = " "; // Whitespace
pub const HDSP: &str = ": "; // Header key separator
pub const EMPT: &str = ""; // Empty separator

#[derive(Debug)]
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

impl HttpMethod {
    fn parse(v: &String) -> Result<Self, HttpParseError> {
        match &v.to_lowercase()[..] {
            "options" => Ok(HttpMethod::Options),
            "get" => Ok(HttpMethod::Get),
            "head" => Ok(HttpMethod::Head),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "delete" => Ok(HttpMethod::Delete),
            "trace" => Ok(HttpMethod::Trace),
            "connect" => Ok(HttpMethod::Connect),
            "patch" => Ok(HttpMethod::Patch),
            _ => Err(HttpParseError::HttpMethodParseErr(format!(
                "expected a valid http method, got: `{}`",
                v
            ))),
        }
    }
}

#[derive(Debug)]
pub enum HttpVersion {
    Http11,
    Http20,
    Http30,
}

impl HttpVersion {
    fn parse(v: &String) -> Result<Self, HttpParseError> {
        match &v.to_lowercase()[..] {
            "http/1.1" => Ok(HttpVersion::Http11),
            "http/2.0" => Ok(HttpVersion::Http20),
            "http/3.0" => Ok(HttpVersion::Http30),
            _ => Err(HttpParseError::HttpMethodParseErr(format!(
                "expected a valid http version, got: `{}`",
                v
            ))),
        }
    }
    fn as_buf(&self) -> Vec<u8> {
        match self {
            Self::Http11 => b"HTTP/1.1".to_vec(),
            Self::Http20 => b"HTTP/2.0".to_vec(),
            Self::Http30 => b"HTTP/3.0".to_vec(),
        }
    }
}

#[derive(Debug)]
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

pub struct HttpLine {
    method: HttpMethod,
    uri: PathBuf,
    version: HttpVersion,
}

impl HttpLine {
    fn parse(tokens: Vec<String>) -> Result<Self, HttpParseError> {
        match tokens.len() {
            3 => Ok(HttpLine {
                method: HttpMethod::parse(&tokens[0])?,
                uri: PathBuf::from(&tokens[1]),
                version: HttpVersion::parse(&tokens[2])?,
            }),
            _ => Err(HttpParseError::HttpRequestLineParseErr(format!(
                "unknown error parsing http request line: tokens: `{:?}`",
                &tokens
            ))),
        }
    }
    fn as_buf(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.append(&mut self.version.as_buf());
        buf.append(&mut self.)
    }
}

#[derive(Debug)]
pub struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    fn parse(tokens: Vec<String>) -> Result<Self, HttpParseError> {
        let mut headers = HashMap::new();
        for token in tokens {
            let mut header = token
                .split(HDSP)
                .map(|v| v.trim().to_string())
                .collect::<Vec<String>>();
            match header.len() {
                2 => {
                    let (value, key) = (header.pop().unwrap(), header.pop().unwrap());
                    let _ = headers.insert(key, value);
                }
                _ => {
                    return Err(HttpParseError::HttpHeaderParseErr(format!(
                        "invalid http header: `{:?}`",
                        header
                    )))
                }
            }
        }

        Ok(HttpHeader { headers })
    }
    fn as_buf(&self) -> Vec<u8> {
        let mut sbuf = String::new();
        for (key, value) in self.headers.iter() {
            sbuf.push(format!("{}: {}{}", key, value, CRLF));
        }
        sbuf.into_bytes()
    }
}

#[derive(Debug)]
pub struct HttpBody {
    tokens: Vec<String>,
}

impl HttpBody {
    fn parse(tokens: Vec<String>) -> Result<Self, HttpParseError> {
        Ok(HttpBody { tokens })
    }
    fn as_buf(&self) -> Vec<u8> {
        self.tokens.join("").into_bytes()
    }
}
