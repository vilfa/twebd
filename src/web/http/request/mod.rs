use crate::web::http::{
    consts,
    err::HttpParseError,
    interop,
    interop::{Parse, TokenIter},
    native::{HttpBody, HttpHeader, HttpLine, HttpRequest},
    HandleRequest, HttpHandler,
};
use std::result::Result;

impl HandleRequest<HttpRequest, HttpParseError> for HttpHandler<HttpRequest> {
    fn handle(buf: &'static mut [u8]) -> Result<HttpRequest, HttpParseError> {
        let sbuf = interop::buffer_to_string(buf)?;
        let mut tokens = interop::string_into_tokens(&sbuf);
        let request_line = parse_request_line(&mut tokens)?;
        let header = parse_header(&mut tokens)?;
        let body = parse_body(&mut tokens)?;
        Ok(HttpRequest {
            method: request_line.method,
            uri: request_line.uri,
            version: request_line.version,
            header: header,
            body: body,
        })
    }
}

fn parse_request_line(tokens: &mut TokenIter) -> Result<HttpLine, HttpParseError> {
    let request_line_tokens = tokens
        .take(1)
        .map(|v| v.split(consts::WSPC))
        .flatten()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    Ok(HttpLine::parse(request_line_tokens)?)
}

fn parse_header(tokens: &mut TokenIter) -> Result<HttpHeader, HttpParseError> {
    let header_tokens = tokens
        .take_while(|&v| v != consts::EMPT)
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    Ok(HttpHeader::parse(header_tokens)?)
}

fn parse_body(tokens: &mut TokenIter) -> Result<HttpBody, HttpParseError> {
    let body_tokens = tokens.map(|v| v.to_string()).collect::<Vec<String>>();
    Ok(HttpBody::parse(body_tokens)?)
}
