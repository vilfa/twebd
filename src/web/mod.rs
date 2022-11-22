pub mod http;
pub mod https;

// TODO: Return default HTTP body responses for any status code.

pub use http::{
    delim,
    err::{HttpParseError, HttpResponseError},
    interop::{buffer_to_string, string_into_tokens, Parse, ToBuf, TokenIter},
    native::{
        HttpBody, HttpHeader, HttpLine, HttpMethod, HttpRequest, HttpResponse, HttpResponseLine,
        HttpStatus, HttpVersion,
    },
    response::get,
    HandleRequest, HandleResponse, HttpHandler,
};
pub use https::{err::TlsConfigError, tls::TlsConfigBuilder};
