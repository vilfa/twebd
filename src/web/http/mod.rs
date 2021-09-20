pub mod get;
pub mod request;
pub mod response;

use crate::{
    web::http::{request::HttpParseError, response::HttpResponseError},
    APP_NAME, APP_VERSION,
};
use chrono::prelude::*;
use std::{collections::HashMap, fmt, path::PathBuf, result::Result};

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
        match &v.to_uppercase()[..] {
            "OPTIONS" => Ok(HttpMethod::Options),
            "GET" => Ok(HttpMethod::Get),
            "HEAD" => Ok(HttpMethod::Head),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "TRACE" => Ok(HttpMethod::Trace),
            "CONNECT" => Ok(HttpMethod::Connect),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(HttpParseError::HttpMethodParseErr(format!(
                "expected a valid http method, got: `{}`",
                v
            ))),
        }
    }
    #[allow(dead_code)]
    fn as_buf(&self) -> Vec<u8> {
        match self {
            Self::Options => b"OPTIONS".to_vec(),
            Self::Get => b"GET".to_vec(),
            Self::Head => b"HEAD".to_vec(),
            Self::Post => b"POST".to_vec(),
            Self::Put => b"PUT".to_vec(),
            Self::Delete => b"DELETE".to_vec(),
            Self::Trace => b"TRACE".to_vec(),
            Self::Connect => b"CONNECT".to_vec(),
            Self::Patch => b"PATCH".to_vec(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HttpVersion {
    Http11,
    Http20,
    Http30,
}

impl HttpVersion {
    fn parse(v: &String) -> Result<Self, HttpParseError> {
        match &v.to_uppercase()[..] {
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            "HTTP/2.0" => Ok(HttpVersion::Http20),
            "HTTP/3.0" => Ok(HttpVersion::Http30),
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

impl Default for HttpVersion {
    fn default() -> Self {
        Self::Http11
    }
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

impl HttpStatus {
    fn as_buf(&self) -> Vec<u8> {
        format!("{}", self).as_bytes().to_vec()
    }
}

impl Default for HttpStatus {
    fn default() -> Self {
        Self::OK
    }
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Continue => write!(f, "{} Continue", (*self as usize)),
            Self::SwitchingProtocols => write!(f, "{} Switching Protocols", (*self as usize)),
            Self::OK => write!(f, "{} OK", (*self as usize)),
            Self::Created => write!(f, "{} Created", (*self as usize)),
            Self::Accepted => write!(f, "{} Accepted", (*self as usize)),
            Self::NonAuthoritativeInformation => {
                write!(f, "{} Non Authoritative Information", (*self as usize))
            }
            Self::NoContent => write!(f, "{} No Content", (*self as usize)),
            Self::ResetContent => write!(f, "{} Reset Content", (*self as usize)),
            Self::PartialContent => write!(f, "{} Partial Content", (*self as usize)),
            Self::MultipleChoices => write!(f, "{} Multiple Choices", (*self as usize)),
            Self::MovedPermanently => write!(f, "{} Moved Permanently", (*self as usize)),
            Self::Found => write!(f, "{} Found", (*self as usize)),
            Self::SeeOther => write!(f, "{} See Other", (*self as usize)),
            Self::NotModified => write!(f, "{} Not Modified", (*self as usize)),
            Self::UseProxy => write!(f, "{} Use Proxy", (*self as usize)),
            Self::TemporaryRedirect => write!(f, "{} Temporary Redirect", (*self as usize)),
            Self::BadRequest => write!(f, "{} Bad Request", (*self as usize)),
            Self::Unauthorized => write!(f, "{} Unauthorized", (*self as usize)),
            Self::Forbidden => write!(f, "{} Forbidden", (*self as usize)),
            Self::NotFound => write!(f, "{} Not Found", (*self as usize)),
            Self::MethodNotAllowed => write!(f, "{} Method Not Allowed", (*self as usize)),
            Self::NotAcceptable => write!(f, "{} Not Acceptable", (*self as usize)),
            Self::ProxyAuthenticationRequired => {
                write!(f, "{} Proxy Authentication Required", (*self as usize))
            }
            Self::RequestTimeout => write!(f, "{} Request Timeout", (*self as usize)),
            Self::Conflict => write!(f, "{} Conflict", (*self as usize)),
            Self::Gone => write!(f, "{} Gone", (*self as usize)),
            Self::LengthRequired => write!(f, "{} Length Required", (*self as usize)),
            Self::PreconditionFailed => write!(f, "{} Precondition Failed", (*self as usize)),
            Self::RequestEntityTooLarge => {
                write!(f, "{} Request Entity Too Large", (*self as usize))
            }
            Self::RequestURITooLong => write!(f, "{} Request URI Too Long", (*self as usize)),
            Self::UnsupportedMediaType => write!(f, "{} Unsupported Media Type", (*self as usize)),
            Self::RequestedRangeNotSatisfiable => {
                write!(f, "{} Requested Range Not Satisfiable", (*self as usize))
            }
            Self::ExpectationFailed => write!(f, "{} Expectation Failed", (*self as usize)),
            Self::InternalServerError => write!(f, "{} Internal Server Error", (*self as usize)),
            Self::NotImplemented => write!(f, "{} Not Implemented", (*self as usize)),
            Self::BadGateway => write!(f, "{} Bad Gateway", (*self as usize)),
            Self::ServiceUnavailable => write!(f, "{} Service Unavailable", (*self as usize)),
            Self::GatewayTimeout => write!(f, "{} Gateway Timeout", (*self as usize)),
            Self::HTTPVersionNotSupported => {
                write!(f, "{} HTTP Version Not Supported", (*self as usize))
            }
        }
    }
}

#[derive(Debug)]
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
    fn _as_buf(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.append(&mut self.method.as_buf());
        buf.append(&mut WSPC.as_bytes().to_vec());
        buf.append(&mut self.uri.to_str().unwrap().as_bytes().to_vec());
        buf.append(&mut WSPC.as_bytes().to_vec());
        buf.append(&mut self.version.as_buf());
        buf.append(&mut CRLF.as_bytes().to_vec());
        buf
    }
}

#[derive(Debug)]
pub struct HttpResponseLine {
    version: HttpVersion,
    status: HttpStatus,
}

impl HttpResponseLine {
    fn new(version: HttpVersion, status: HttpStatus) -> HttpResponseLine {
        HttpResponseLine { version, status }
    }
    fn as_buf(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.append(&mut self.version.as_buf());
        buf.append(&mut WSPC.as_bytes().to_vec());
        buf.append(&mut self.status.as_buf());
        buf.append(&mut CRLF.as_bytes().to_vec());
        buf
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
            sbuf.push_str(&format!("{}: {}{}", &key, &value, CRLF));
        }
        sbuf.push_str(CRLF);
        sbuf.into_bytes()
    }
}

impl Default for HttpHeader {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Date"), Utc::now().to_rfc3339());
        headers.insert(
            String::from("Server"),
            format!("{}/{}", APP_NAME, APP_VERSION),
        );
        HttpHeader { headers }
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

impl From<String> for HttpBody {
    fn from(v: String) -> Self {
        HttpBody { tokens: vec![v] }
    }
}

impl From<HttpResponseError> for HttpBody {
    fn from(v: HttpResponseError) -> Self {
        match v {
            HttpResponseError::FileReaderError(e) => HttpBody {
                tokens: vec![format!("{:?}", e)],
            },
            HttpResponseError::FilePathInvalid(e) => HttpBody { tokens: vec![e] },
            HttpResponseError::FileNotFound(e) => HttpBody { tokens: vec![e] },
        }
    }
}

impl Default for HttpBody {
    fn default() -> Self {
        HttpBody { tokens: Vec::new() }
    }
}
