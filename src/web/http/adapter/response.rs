use crate::web::{
    handle::get, HttpAdapter, HttpMethod, HttpRequest, HttpResponder, HttpResponse, HttpStatus,
};
use std::path::PathBuf;

impl HttpResponder<HttpResponse> for HttpAdapter<HttpResponse> {
    fn respond(request: &HttpRequest, srv_root: &PathBuf) -> HttpResponse {
        match request.method {
            HttpMethod::Get => get(&request.uri, srv_root),
            _ => {
                let mut response = HttpResponse::default();
                response.status = HttpStatus::NotImplemented;
                response
            }
        }
    }
}
