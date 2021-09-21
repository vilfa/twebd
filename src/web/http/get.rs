use crate::{
    srv::file::{File, FileReader},
    web::http::err::HttpResponseError,
};
use std::path::PathBuf;

pub struct HttpGetHandler<'a> {
    uri: &'a PathBuf,
    srv_root: PathBuf,
}

impl<'a> HttpGetHandler<'a> {
    pub fn new(uri: &'a PathBuf, srv_root: &'a PathBuf) -> HttpGetHandler<'a> {
        HttpGetHandler {
            uri,
            srv_root: srv_root.canonicalize().unwrap(),
        }
    }
    pub fn handle(&self) -> Result<File, HttpResponseError> {
        let uri = self.sanitize()?;
        let file = FileReader::new(&uri).read()?;
        Ok(file)
    }
    fn sanitize(&self) -> Result<PathBuf, HttpResponseError> {
        let uri = self.uri_abs()?;
        if uri.starts_with(&self.srv_root) {
            Ok(uri)
        } else {
            Err(HttpResponseError::FilePathInvalid(format!(
                "the requested file is not in the server root: `{:?}`",
                uri
            )))
        }
    }
    fn uri_abs(&self) -> Result<PathBuf, HttpResponseError> {
        let uri_rel = if self.uri == &PathBuf::from("/") {
            self.srv_root.join(PathBuf::from("index.html"))
        } else {
            self.srv_root
                .join(PathBuf::from(format!(".{}", self.uri.to_str().unwrap())))
        };

        if let Ok(uri_abs) = uri_rel.canonicalize() {
            Ok(uri_abs)
        } else {
            Err(HttpResponseError::FileNotFound(format!(
                "the requested file was not found on this server: `{:?}`",
                &self.uri
            )))
        }
    }
}
