use super::{HttpBody, HttpHeader, HttpLine, HttpMethod, HttpVersion, CRLF, EMPT, WSPC};
use crate::log::{backlog::Backlog, LogRecord};
use std::{path::PathBuf, result::Result};

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    uri: PathBuf,
    version: HttpVersion,
    header: HttpHeader,
    body: HttpBody,
}

pub struct HttpRequestParser<'a> {
    _buf: &'a [u8],
    sbuf: String,
    backlog: Vec<LogRecord>,
}

impl<'a> HttpRequestParser<'a> {
    pub fn new(buf: &'a mut [u8]) -> Result<HttpRequestParser, HttpParseError> {
        let sbuf = HttpRequestParser::buf_to_str(buf)?;
        Ok(HttpRequestParser {
            _buf: buf,
            sbuf,
            backlog: Vec::new(),
        })
    }
    pub fn request(&self) -> Result<HttpRequest, HttpParseError> {
        let mut tokens = self.iter();
        let request_line = HttpRequestParser::request_line(&mut tokens)?;
        let header = HttpRequestParser::header(&mut tokens)?;
        let body = HttpRequestParser::body(&mut tokens)?;
        Ok(HttpRequest {
            method: request_line.method,
            uri: request_line.uri,
            version: request_line.version,
            header: header,
            body: body,
        })
    }
    fn buf_to_str(buf: &'a [u8]) -> Result<String, HttpParseError> {
        match std::str::from_utf8(buf) {
            Ok(v) => match regex::Regex::new(" +") {
                Ok(r) => {
                    let sbuf = r.replace_all(v.trim(), WSPC).to_string();
                    Ok(sbuf)
                }
                Err(e) => Err(HttpParseError::ParserInitError(format!(
                    "error creating http request parser: `{:?}`",
                    e
                ))),
            },
            Err(e) => Err(HttpParseError::ParserInitError(format!(
                "error creating http request parser: `{:?}`",
                e
            ))),
        }
    }
    fn iter(&self) -> TokenIter {
        self.sbuf
            .split(CRLF)
            .map(|v| v.trim())
            .collect::<Vec<&str>>()
            .into_iter()
    }
    fn request_line(tokens: &mut TokenIter) -> Result<HttpLine, HttpParseError> {
        let request_line_tokens = tokens
            .take(1)
            .map(|v| v.split(WSPC))
            .flatten()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        Ok(HttpLine::parse(request_line_tokens)?)
    }
    fn header(tokens: &mut TokenIter) -> Result<HttpHeader, HttpParseError> {
        let header_tokens = tokens
            .take_while(|&v| v != EMPT)
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        Ok(HttpHeader::parse(header_tokens)?)
    }
    fn body(tokens: &mut TokenIter) -> Result<HttpBody, HttpParseError> {
        let body_tokens = tokens.map(|v| v.to_string()).collect::<Vec<String>>();
        Ok(HttpBody::parse(body_tokens)?)
    }
}

impl<'a> Backlog for HttpRequestParser<'a> {
    fn backlog(&self) -> Vec<LogRecord> {
        self.backlog.to_vec()
    }
}

#[derive(Debug)]
pub enum HttpParseError {
    ParserInitError(String),
    HttpVersionParseErr(String),
    HttpMethodParseErr(String),
    HttpHeaderParseErr(String),
    HttpRequestLineParseErr(String),
}

type TokenIter<'a> = std::vec::IntoIter<&'a str>;
