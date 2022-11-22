pub mod handle;
pub mod http;
pub mod https;

// TODO: Return default HTTP body responses for any status code.

pub use http::{
    adapter::{HttpAcceptor, HttpAdapter, HttpResponder},
    delim,
    err::{HttpParseError, HttpResponseError},
    interop::{stringify, tokenize_s, HttpParser, ToBuffer, TokenIter},
    native::{
        HttpBody, HttpHeader, HttpLine, HttpMethod, HttpRequest, HttpResponse, HttpResponseLine,
        HttpStatus, HttpVersion,
    },
};
pub use https::{err::TlsConfigError, tls::TlsConfigBuilder};
