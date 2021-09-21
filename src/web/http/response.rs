use crate::{
    log::{backlog::Backlog, LogRecord},
    web::http::{
        get::HttpGetHandler,
        native::{HttpBody, HttpMethod, HttpRequest, HttpResponse, HttpStatus},
    },
};
use std::path::PathBuf;

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
                response.header.headers.insert(
                    String::from("Content-Type"),
                    String::from(v.mime().essence_str()),
                );
                response
                    .header
                    .headers
                    .insert(String::from("Content-Length"), format!("{}", v.size()));
                response.body = HttpBody::new(v.as_string());
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
