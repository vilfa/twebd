use crate::web::http::delim;
use crate::web::{HttpBody, HttpHeader, HttpLine, HttpMethod, HttpParseError, HttpVersion, Parse};
use std::{collections::HashMap, path::PathBuf, result::Result};

type E = HttpParseError;

impl Parse<Vec<String>, Self, E> for HttpBody {
    fn parse(v: Vec<String>) -> Result<Self, E> {
        Ok(Self { tokens: v })
    }
}

impl Parse<Vec<String>, Self, E> for HttpHeader {
    fn parse(v: Vec<String>) -> Result<Self, E> {
        let mut headers = HashMap::new();
        for token in v {
            let mut header = token
                .split(delim::CLSP)
                .map(|v| v.trim().to_string())
                .collect::<Vec<String>>();
            match header.len() {
                2 => {
                    let (value, key) = (header.pop().unwrap(), header.pop().unwrap());
                    let _ = headers.insert(key, value);
                }
                _ => {
                    return Err(HttpParseError::HttpHeader(format!(
                        "invalid http header: {:?}",
                        header
                    )))
                }
            }
        }

        Ok(HttpHeader { headers })
    }
}

impl Parse<Vec<String>, Self, E> for HttpLine {
    fn parse(v: Vec<String>) -> Result<Self, E> {
        match v.len() {
            3 => Ok(HttpLine {
                method: HttpMethod::parse(&v[0])?,
                uri: PathBuf::from(&v[1]),
                version: HttpVersion::parse(&v[2])?,
            }),
            _ => Err(HttpParseError::HttpRequestLine(format!(
                "unknown error parsing http request line: {:?}",
                &v
            ))),
        }
    }
}

impl Parse<&String, Self, E> for HttpMethod {
    fn parse(v: &String) -> Result<Self, E> {
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
            _ => Err(E::HttpMethod(format!(
                "expected a valid http method, got: {}",
                v
            ))),
        }
    }
}

impl Parse<&String, Self, E> for HttpVersion {
    fn parse(v: &String) -> Result<Self, E> {
        match &v.to_uppercase()[..] {
            "HTTP/1.0" => Ok(HttpVersion::Http11),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            "HTTP/2.0" => Ok(HttpVersion::Http20),
            _ => Err(HttpParseError::HttpMethod(format!(
                "expected a valid http version, got: {}",
                v
            ))),
        }
    }
}
