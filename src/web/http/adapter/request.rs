use crate::web::{
    http::delim, stringify, tokenize_s, HttpAcceptor, HttpAdapter, HttpBody, HttpHeader, HttpLine,
    HttpParseError, HttpParser, HttpRequest,
};
use std::result::Result;

impl HttpAcceptor<HttpRequest, HttpParseError> for HttpAdapter<HttpRequest> {
    fn accept(buf: &mut [u8]) -> Result<HttpRequest, HttpParseError> {
        let sbuf = stringify(buf)?;
        let mut tokens = tokenize_s(&sbuf);

        let req_line = HttpLine::parse(
            (&mut tokens)
                .take(1)
                .map(|v| v.split(delim::WSPC))
                .flatten()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        )?;

        let header = HttpHeader::parse(
            (&mut tokens)
                .take_while(|&v| v != delim::EMPT)
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        )?;

        let body = HttpBody::parse(
            (&mut tokens)
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        )?;

        Ok(HttpRequest {
            method: req_line.method,
            uri: req_line.uri,
            version: req_line.version,
            header,
            body,
        })
    }
}
