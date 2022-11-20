use crate::web::HttpBody;

#[derive(Debug)]
pub enum HttpResponseError {
    FileIo(std::io::Error),
    FilePathInvalid(String),
    FileNotFound(String),
}

impl From<std::io::Error> for HttpResponseError {
    fn from(e: std::io::Error) -> Self {
        HttpResponseError::FileIo(e)
    }
}

impl From<HttpResponseError> for HttpBody {
    fn from(v: HttpResponseError) -> Self {
        match v {
            HttpResponseError::FileIo(e) => HttpBody {
                tokens: vec![format!("{:?}", e)],
            },
            HttpResponseError::FilePathInvalid(e) => HttpBody { tokens: vec![e] },
            HttpResponseError::FileNotFound(e) => HttpBody { tokens: vec![e] },
        }
    }
}

#[derive(Debug)]
pub enum HttpParseError {
    Buffer(String),
    HttpVersion(String),
    HttpMethod(String),
    HttpHeader(String),
    HttpRequestLine(String),
}
