use super::{HttpBody, HttpHeader, HttpMethod, HttpReqLine, HttpRequest, HttpVersion};
use std::{io::Result, path::PathBuf};

pub fn parse_request(buf: &mut [u8]) -> Result<HttpRequest> {
    let request_line = parse_request_line(buf)?;
    let header = parse_header(buf)?;
    let body = parse_body(buf)?;

    Ok(HttpRequest {
        method: HttpMethod::Get,
        uri: PathBuf::from("/"),
        version: HttpVersion::Http11,
        header: HttpHeader {},
        body: HttpBody {},
    })
}

fn parse_request_line(buf: &[u8]) -> Result<HttpReqLine> {
    Ok(HttpReqLine {})
}

fn parse_header(buf: &[u8]) -> Result<HttpHeader> {
    Ok(HttpHeader {})
}

fn parse_body(buf: &[u8]) -> Result<HttpBody> {
    Ok(HttpBody {})
}
