pub mod get;

use crate::web::{
    get::get, HandleResponse, HttpHandler, HttpMethod, HttpRequest, HttpResponse, HttpStatus,
};
use std::path::PathBuf;

impl HandleResponse<HttpResponse> for HttpHandler<HttpResponse> {
    fn handle(request: &HttpRequest, srv_root: &PathBuf) -> HttpResponse {
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
