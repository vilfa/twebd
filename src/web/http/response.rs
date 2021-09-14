use crate::{
    log::{backlog::Backlog, LogRecord},
    web::http::{
        get::HttpGetHandler, request::HttpRequest, HttpBody, HttpHeader, HttpMethod,
        HttpResponseLine, HttpStatus, HttpVersion,
    },
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct HttpResponse {
    version: HttpVersion,
    status: HttpStatus,
    header: HttpHeader,
    body: HttpBody,
}

impl<'a> HttpResponse {
    pub fn as_buf(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let bufs = vec![
            HttpResponseLine::new(self.version, self.status).as_buf(),
            self.header.as_buf(),
            self.body.as_buf(),
        ];
        for mut b in bufs {
            buf.append(&mut b);
        }
        buf
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        HttpResponse {
            version: HttpVersion::default(),
            status: HttpStatus::default(),
            header: HttpHeader::default(),
            body: HttpBody::default(),
        }
    }
}

pub struct HttpResponseBuilder<'a> {
    request: &'a HttpRequest,
    srv_root: &'a PathBuf,
    backlog: Vec<LogRecord>,
}

impl<'a> HttpResponseBuilder<'a> {
    pub fn new(request: &'a HttpRequest, srv_root: &'a PathBuf) -> HttpResponseBuilder<'a> {
        HttpResponseBuilder {
            request,
            srv_root,
            backlog: Vec::new(),
        }
    }
    pub fn response(&self) -> HttpResponse {
        match self.request.method {
            HttpMethod::Get => self.get(),
            _ => {
                let mut response = HttpResponse::default();
                response.status = HttpStatus::NotImplemented;
                response
            }
        }
    }
    fn get(&self) -> HttpResponse {
        match HttpGetHandler::new(&self.request.uri, &self.srv_root).handle() {
            Ok(v) => {
                let mut response = HttpResponse::default();
                response.body = HttpBody::from(v);
                response
            }
            Err(e) => {
                let mut response = HttpResponse::default();
                response.status = HttpStatus::NotFound;
                response.body = HttpBody::from(e);
                response
            }
        }
    }
}

impl Backlog for HttpResponseBuilder<'_> {
    fn backlog(&self) -> Vec<LogRecord> {
        self.backlog.to_vec()
    }
}

#[derive(Debug)]
pub enum HttpResponseError {
    FileReaderError(std::io::Error),
    FilePathInvalid(String),
    FileNotFound(String),
}
