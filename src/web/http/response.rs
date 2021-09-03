use super::{request::HttpRequest, HttpBody, HttpHeader, HttpStatus, HttpVersion};

pub struct HttpResponse {
    version: HttpVersion,
    status: HttpStatus,
    header: HttpHeader,
    body: HttpBody,
}

pub struct HttpResponseBuilder<'a> {
    request: &'a HttpRequest,
}

impl<'a> HttpResponseBuilder<'a> {
    pub fn new(request: &'a HttpRequest) -> HttpResponseBuilder {
        HttpResponseBuilder { request }
    }
    pub fn response(&self) -> HttpResponse {
        HttpResponse {
            version: HttpVersion::Http11,
            status: HttpStatus::OK,
            header: HttpHeader {
                headers: std::collections::HashMap::new(),
            },
            body: HttpBody { tokens: Vec::new() },
        }
    }
}
