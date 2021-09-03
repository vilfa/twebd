pub mod request;
pub mod response;

use crate::web::http::request::HttpParseError;
use std::{collections::HashMap, path::PathBuf, result::Result};

pub const CRLF: &str = "\r\n";
pub const WSPC: &str = " ";

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
    fn parse(v: String) -> Result<Self, HttpParseError> {
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

pub enum HttpVersion {
    Http11,
    Http20,
    Http30,
}

impl HttpVersion {
    fn parse(v: String) -> Result<Self, HttpParseError> {
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
}

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
                method: HttpMethod::parse(tokens[0])?,
                uri: PathBuf::from(tokens[1]),
                version: HttpVersion::parse(tokens[2])?,
            }),
            _ => Err(HttpParseError::HttpRequestLineParseErr(format!(
                "unknown error parsing http request line"
            ))),
        }
    }
}

pub struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    fn parse(tokens: Vec<String>) -> Result<Self, HttpParseError> {
        let mut headers = HashMap::new();
        let mut line: Vec<String> = Vec::new();
        for token in tokens {
            if token == CRLF {
                if line.len() >= 2 {
                    headers.insert(line[0].replace(":", ""), line[1..].join(""));
                } else {
                    return Err(HttpParseError::HttpHeaderParseErr(format!(
                        "invalid http header: `{:?}`",
                        line
                    )));
                }
            } else {
                line.push(token);
            }
        }

        Ok(HttpHeader { headers })
    }
}

pub struct HttpBody {
    tokens: Vec<String>,
}

impl HttpBody {
    fn parse(tokens: Vec<String>) -> Result<Self, HttpParseError> {
        Ok(HttpBody { tokens })
    }
}
