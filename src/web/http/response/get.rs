use crate::{
    srv::file::FileReader,
    web::http::{
        err::HttpResponseError,
        native::{HttpBody, HttpResponse, HttpStatus},
    },
};
use std::path::PathBuf;

pub fn get(uri: &PathBuf, srv_root: &PathBuf) -> HttpResponse {
    let root = srv_root.canonicalize().unwrap();
    let uri = match sanitize_uri(uri, &root) {
        Ok(v) => v,
        Err(e) => {
            let mut response = HttpResponse::default();
            response.status = HttpStatus::NotFound;
            response.body = HttpBody::from(e);
            return response;
        }
    };

    let resource = match FileReader::new(&uri).read() {
        Ok(v) => v,
        Err(e) => {
            let mut response = HttpResponse::default();
            response.status = HttpStatus::InternalServerError;
            response.body = HttpBody::from(e);
            return response;
        }
    };

    let mut response = HttpResponse::default();
    response.add_header(
        String::from("Content-Type"),
        String::from(resource.mime().essence_str()),
    );
    response.add_header(
        String::from("Content-Length"),
        format!("{}", resource.size()),
    );
    response.body = HttpBody::new(resource.as_string());
    response
}

fn sanitize_uri(uri: &PathBuf, srv_root: &PathBuf) -> Result<PathBuf, HttpResponseError> {
    let uri = absolute_uri(uri, srv_root)?;
    if uri.starts_with(srv_root) {
        Ok(uri)
    } else {
        Err(HttpResponseError::FilePathInvalid(format!(
            "the requested file is not in the server root: `{:?}`",
            uri
        )))
    }
}

fn absolute_uri(uri: &PathBuf, srv_root: &PathBuf) -> Result<PathBuf, HttpResponseError> {
    let relative_uri = if uri == &PathBuf::from("/") {
        srv_root.join(PathBuf::from("index.html"))
    } else {
        srv_root.join(PathBuf::from(format!(".{}", uri.to_str().unwrap())))
    };

    relative_uri.canonicalize().or_else(|_| {
        Err(HttpResponseError::FileNotFound(format!(
            "the requested file was not found on this server: `{:?}`",
            uri
        )))
    })
}
