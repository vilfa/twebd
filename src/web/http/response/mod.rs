pub mod get;

use crate::web::http::{
    native::{HttpMethod, HttpRequest, HttpResponse, HttpStatus},
    response, HandleResponse, HttpHandler,
};
use std::path::PathBuf;

impl HandleResponse<HttpResponse> for HttpHandler {
    fn handle(request: &HttpRequest, srv_root: &PathBuf) -> HttpResponse {
        match request.method {
            HttpMethod::Get => response::get::get(&request.uri, srv_root),
            _ => {
                let mut response = HttpResponse::default();
                response.status = HttpStatus::NotImplemented;
                response
            }
        }
    }
}
