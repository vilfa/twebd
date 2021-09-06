use super::{request::HttpRequest, HttpBody, HttpHeader, HttpStatus, HttpVersion};
use crate::log::{backlog::Backlog, LogRecord};

#[derive(Debug)]
pub struct HttpResponse {
    version: HttpVersion,
    status: HttpStatus,
    header: HttpHeader,
    body: HttpBody,
}

impl<'a> HttpResponse {
    pub fn as_buf(&self) -> &'a [u8] {
        &[0; 1]
    }
}

pub struct HttpResponseBuilder<'a> {
    request: &'a HttpRequest,
    backlog: Vec<LogRecord>,
}

impl<'a> HttpResponseBuilder<'a> {
    pub fn new(request: &'a HttpRequest) -> HttpResponseBuilder {
        HttpResponseBuilder {
            request,
            backlog: Vec::new(),
        }
    }
    pub fn response(&self) -> Result<HttpResponse, HttpResponseError> {
        Ok(HttpResponse {
            version: HttpVersion::Http11,
            status: HttpStatus::OK,
            header: HttpHeader {
                headers: std::collections::HashMap::new(),
            },
            body: HttpBody { tokens: Vec::new() },
        })
    }
}

impl Backlog for HttpResponseBuilder<'_> {
    fn backlog(&self) -> Vec<LogRecord> {
        self.backlog.to_vec()
    }
}

#[derive(Debug)]
pub enum HttpResponseError {}
