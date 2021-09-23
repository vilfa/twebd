use crate::web::HttpBody;

#[derive(Debug)]
pub enum HttpResponseError {
    FileIoError(std::io::Error),
    FilePathInvalid(String),
    FileNotFound(String),
}

impl From<std::io::Error> for HttpResponseError {
    fn from(e: std::io::Error) -> Self {
        HttpResponseError::FileIoError(e)
    }
}

impl From<HttpResponseError> for HttpBody {
    fn from(v: HttpResponseError) -> Self {
        match v {
            HttpResponseError::FileIoError(e) => HttpBody {
                tokens: vec![format!("{:?}", e)],
            },
            HttpResponseError::FilePathInvalid(e) => HttpBody { tokens: vec![e] },
            HttpResponseError::FileNotFound(e) => HttpBody { tokens: vec![e] },
        }
    }
}

#[derive(Debug)]
pub enum HttpParseError {
    BufferParseError(String),
    HttpVersionParseErr(String),
    HttpMethodParseErr(String),
    HttpHeaderParseErr(String),
    HttpRequestLineParseErr(String),
}
