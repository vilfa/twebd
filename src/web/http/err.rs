use crate::web::http::native::HttpBody;

#[derive(Debug)]
pub enum HttpResponseError {
    FileReaderError(std::io::Error),
    FilePathInvalid(String),
    FileNotFound(String),
}

impl From<HttpResponseError> for HttpBody {
    fn from(v: HttpResponseError) -> Self {
        match v {
            HttpResponseError::FileReaderError(e) => HttpBody {
                tokens: vec![format!("{:?}", e)],
            },
            HttpResponseError::FilePathInvalid(e) => HttpBody { tokens: vec![e] },
            HttpResponseError::FileNotFound(e) => HttpBody { tokens: vec![e] },
        }
    }
}

#[derive(Debug)]
pub enum HttpParseError {
    ParserInitError(String),
    HttpVersionParseErr(String),
    HttpMethodParseErr(String),
    HttpHeaderParseErr(String),
    HttpRequestLineParseErr(String),
}
