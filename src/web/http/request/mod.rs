use crate::web::http::delim;
use crate::web::{
    buffer_to_string, string_into_tokens, HandleRequest, HttpBody, HttpHandler, HttpHeader,
    HttpLine, HttpParseError, HttpRequest, Parse, TokenIter,
};
use log::trace;
use std::result::Result;

impl HandleRequest<HttpRequest, HttpParseError> for HttpHandler<HttpRequest> {
    fn handle(buf: &mut [u8]) -> Result<HttpRequest, HttpParseError> {
        trace!("called handle request");
        let sbuf = buffer_to_string(buf)?;
        let mut tokens = string_into_tokens(&sbuf);
        let request_line = parse_request_line(&mut tokens)?;
        let header = parse_header(&mut tokens)?;
        let body = parse_body(&mut tokens)?;
        trace!("finished parsing request");
        Ok(HttpRequest {
            method: request_line.method,
            uri: request_line.uri,
            version: request_line.version,
            header,
            body,
        })
    }
}

fn parse_request_line(tokens: &mut TokenIter) -> Result<HttpLine, HttpParseError> {
    let request_line_tokens = tokens
        .take(1)
        .map(|v| v.split(delim::WSPC))
        .flatten()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    Ok(HttpLine::parse(request_line_tokens)?)
}

fn parse_header(tokens: &mut TokenIter) -> Result<HttpHeader, HttpParseError> {
    let header_tokens = tokens
        .take_while(|&v| v != delim::EMPT)
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    Ok(HttpHeader::parse(header_tokens)?)
}

fn parse_body(tokens: &mut TokenIter) -> Result<HttpBody, HttpParseError> {
    let body_tokens = tokens.map(|v| v.to_string()).collect::<Vec<String>>();
    Ok(HttpBody::parse(body_tokens)?)
}
