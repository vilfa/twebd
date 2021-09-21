use crate::{
    log::{backlog::Backlog, LogRecord},
    web::http::{
        consts,
        err::HttpParseError,
        interop::Parse,
        native::{HttpBody, HttpHeader, HttpLine, HttpRequest},
    },
};
use std::result::Result;

type TokenIter<'a> = std::vec::IntoIter<&'a str>;

pub struct HttpRequestParser {
    buf: String,
    _backlog: Vec<LogRecord>,
}

impl HttpRequestParser {
    pub fn new(buf: &'static mut [u8]) -> Result<HttpRequestParser, HttpParseError> {
        let buf = HttpRequestParser::buf_to_str(buf)?;
        Ok(HttpRequestParser {
            buf,
            _backlog: Vec::new(),
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
    fn buf_to_str(buf: &[u8]) -> Result<String, HttpParseError> {
        match std::str::from_utf8(buf) {
            Ok(v) => match regex::Regex::new(" +") {
                Ok(r) => {
                    let sbuf = r.replace_all(v.trim(), consts::WSPC).to_string();
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
        self.buf
            .split(consts::CRLF)
            .map(|v| v.trim())
            .collect::<Vec<&str>>()
            .into_iter()
    }
    fn request_line(tokens: &mut TokenIter) -> Result<HttpLine, HttpParseError> {
        let request_line_tokens = tokens
            .take(1)
            .map(|v| v.split(consts::WSPC))
            .flatten()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        Ok(HttpLine::parse(request_line_tokens)?)
    }
    fn header(tokens: &mut TokenIter) -> Result<HttpHeader, HttpParseError> {
        let header_tokens = tokens
            .take_while(|&v| v != consts::EMPT)
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        Ok(HttpHeader::parse(header_tokens)?)
    }
    fn body(tokens: &mut TokenIter) -> Result<HttpBody, HttpParseError> {
        let body_tokens = tokens.map(|v| v.to_string()).collect::<Vec<String>>();
        Ok(HttpBody::parse(body_tokens)?)
    }
}

impl Backlog for HttpRequestParser {
    fn backlog(&self) -> Vec<LogRecord> {
        self._backlog.to_vec()
    }
}
