use super::{HttpBody, HttpHeader, HttpLine, HttpMethod, HttpVersion, CRLF, WSPC};
use std::{path::PathBuf, result::Result};

pub struct HttpRequest {
    method: HttpMethod,
    uri: PathBuf,
    version: HttpVersion,
    header: HttpHeader,
    body: HttpBody,
}

pub struct HttpRequestParser<'a> {
    buf: &'a [u8],
    sbuf: String,
    stage: HttpParseStage,
    crlf: String,
    wspc: String,
}

impl<'a> HttpRequestParser<'a> {
    pub fn new(buf: &'a mut [u8]) -> Result<HttpRequestParser, HttpParseError> {
        let sbuf = HttpRequestParser::buf_to_str(buf)?;
        Ok(HttpRequestParser {
            buf,
            sbuf,
            stage: HttpParseStage::Ready,
            crlf: String::from(CRLF),
            wspc: String::from(WSPC),
        })
    }
    fn buf_to_str(buf: &'a [u8]) -> Result<String, HttpParseError> {
        match std::str::from_utf8(buf) {
            Ok(v) => match regex::Regex::new(" +") {
                Ok(r) => {
                    let sbuf = r.replace_all(v.trim(), " ").to_string();
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
    // fn iter(&'a mut self) -> RequestBufferTokens<'a, String> {
    fn iter(&'a mut self) -> std::iter::Peekable<std::slice::Iter<'a, String>> {
        self.sbuf
            .split(" ")
            .map(|v| v.trim().to_string())
            .collect::<Vec<String>>()
            .iter()
            .peekable()
    }
    pub fn request(&'a mut self) -> Result<HttpRequest, HttpParseError> {
        let tokens = self.iter();
        let request_line = self.request_line(&mut tokens)?;
        let header = self.header(&mut tokens)?;
        let body = self.body(&mut tokens)?;
        Ok(HttpRequest {
            method: request_line.method,
            uri: request_line.uri,
            version: request_line.version,
            header: header,
            body: body,
        })
    }
    fn request_line(&'a mut self, tokens: &mut TokenIter<'a>) -> Result<HttpLine, HttpParseError> {
        self.stage = HttpParseStage::RequestLine;
        let request_line_tokens = tokens
            .take_while(|v| **v != CRLF)
            .collect::<Vec<&String>>()
            .iter()
            .map(|v| **v)
            .collect::<Vec<String>>();
        Ok(HttpLine::parse(request_line_tokens)?)
    }
    fn header(&'a mut self, tokens: &mut TokenIter<'a>) -> Result<HttpHeader, HttpParseError> {
        self.stage = HttpParseStage::Header;
        // let header_tokens = tokens.take_while(predicate: P);
        let header_tokens = Vec::new();
        for token in tokens {
            if token == &self.wspc && tokens.peek() == Some(&&self.wspc) {
                tokens.take(2);
                break;
            } else {
                header_tokens.push(*token);
            }
        }
        Ok(HttpHeader::parse(header_tokens)?)
    }
    fn body(&'a mut self, tokens: &mut TokenIter<'a>) -> Result<HttpBody, HttpParseError> {
        self.stage = HttpParseStage::Body;
        let body_tokens = tokens
            .collect::<Vec<&String>>()
            .iter()
            .map(|v| **v)
            .collect::<Vec<String>>();
        Ok(HttpBody::parse(body_tokens)?)
    }
}

enum HttpParseStage {
    Ready,
    RequestLine,
    Header,
    Body,
    Done,
}

#[derive(Debug)]
pub enum HttpParseError {
    ParserInitError(String),
    HttpVersionParseErr(String),
    HttpMethodParseErr(String),
    HttpHeaderParseErr(String),
    HttpRequestLineParseErr(String),
}

type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, String>>;

// struct RequestBufferTokens<'a, T>(std::iter::Peekable<std::slice::Iter<'a, T>>);
//
// impl<'a, T> Iterator for RequestBufferTokens<'a, T> {
// type Item = &'a T;
// fn next(&mut self) -> Option<Self::Item> {
// self.0.next()
// }
// }
