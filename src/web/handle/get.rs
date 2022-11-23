use crate::{
    srv::FileReader,
    web::{HttpBody, HttpResponse, HttpResponseError, HttpStatus},
};
use log::{debug, trace};
use std::path::PathBuf;

pub fn get(uri: &PathBuf, srv_root: &PathBuf) -> HttpResponse {
    debug!("get {:?} with root {:?}", &uri, &srv_root);
    let root = srv_root.canonicalize().unwrap();
    let uri = match sanitize_uri(uri, &root) {
        Ok(v) => v,
        Err(e) => {
            debug!("the requested resource could not be found: {:?}", e);
            return HttpResponse::from(HttpStatus::NotFound);
        }
    };

    let resource = match FileReader::new(&uri).read() {
        Ok(v) => v,
        Err(e) => {
            debug!("failed to load the requested resource: {:?}", e);
            return HttpResponse::from(HttpStatus::InternalServerError);
        }
    };

    let mut response = HttpResponse::default();
    response.content_type(resource.mime().essence_str().to_owned());
    response.content_length(resource.size());
    response.body = HttpBody::new(resource.as_string());

    trace!("built response: {:?}", &response);
    response
}

fn sanitize_uri(uri: &PathBuf, srv_root: &PathBuf) -> Result<PathBuf, HttpResponseError> {
    let uri = absolute_uri(uri, srv_root)?;
    if uri.starts_with(srv_root) {
        Ok(uri)
    } else {
        Err(HttpResponseError::FilePathInvalid(format!(
            "the requested resource is not in the server root: {:?}",
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
            "the requested resource was not found on this server: {:?}",
            uri
        )))
    })
}
