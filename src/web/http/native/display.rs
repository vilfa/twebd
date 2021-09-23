use crate::web::HttpStatus;
use std::fmt::{Display, Formatter, Result};

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Continue => write!(f, "{} Continue", (*self as usize)),
            Self::SwitchingProtocols => write!(f, "{} Switching Protocols", (*self as usize)),
            Self::OK => write!(f, "{} OK", (*self as usize)),
            Self::Created => write!(f, "{} Created", (*self as usize)),
            Self::Accepted => write!(f, "{} Accepted", (*self as usize)),
            Self::NonAuthoritativeInformation => {
                write!(f, "{} Non Authoritative Information", (*self as usize))
            }
            Self::NoContent => write!(f, "{} No Content", (*self as usize)),
            Self::ResetContent => write!(f, "{} Reset Content", (*self as usize)),
            Self::PartialContent => write!(f, "{} Partial Content", (*self as usize)),
            Self::MultipleChoices => write!(f, "{} Multiple Choices", (*self as usize)),
            Self::MovedPermanently => write!(f, "{} Moved Permanently", (*self as usize)),
            Self::Found => write!(f, "{} Found", (*self as usize)),
            Self::SeeOther => write!(f, "{} See Other", (*self as usize)),
            Self::NotModified => write!(f, "{} Not Modified", (*self as usize)),
            Self::UseProxy => write!(f, "{} Use Proxy", (*self as usize)),
            Self::TemporaryRedirect => write!(f, "{} Temporary Redirect", (*self as usize)),
            Self::BadRequest => write!(f, "{} Bad Request", (*self as usize)),
            Self::Unauthorized => write!(f, "{} Unauthorized", (*self as usize)),
            Self::Forbidden => write!(f, "{} Forbidden", (*self as usize)),
            Self::NotFound => write!(f, "{} Not Found", (*self as usize)),
            Self::MethodNotAllowed => write!(f, "{} Method Not Allowed", (*self as usize)),
            Self::NotAcceptable => write!(f, "{} Not Acceptable", (*self as usize)),
            Self::ProxyAuthenticationRequired => {
                write!(f, "{} Proxy Authentication Required", (*self as usize))
            }
            Self::RequestTimeout => write!(f, "{} Request Timeout", (*self as usize)),
            Self::Conflict => write!(f, "{} Conflict", (*self as usize)),
            Self::Gone => write!(f, "{} Gone", (*self as usize)),
            Self::LengthRequired => write!(f, "{} Length Required", (*self as usize)),
            Self::PreconditionFailed => write!(f, "{} Precondition Failed", (*self as usize)),
            Self::RequestEntityTooLarge => {
                write!(f, "{} Request Entity Too Large", (*self as usize))
            }
            Self::RequestURITooLong => write!(f, "{} Request URI Too Long", (*self as usize)),
            Self::UnsupportedMediaType => write!(f, "{} Unsupported Media Type", (*self as usize)),
            Self::RequestedRangeNotSatisfiable => {
                write!(f, "{} Requested Range Not Satisfiable", (*self as usize))
            }
            Self::ExpectationFailed => write!(f, "{} Expectation Failed", (*self as usize)),
            Self::InternalServerError => write!(f, "{} Internal Server Error", (*self as usize)),
            Self::NotImplemented => write!(f, "{} Not Implemented", (*self as usize)),
            Self::BadGateway => write!(f, "{} Bad Gateway", (*self as usize)),
            Self::ServiceUnavailable => write!(f, "{} Service Unavailable", (*self as usize)),
            Self::GatewayTimeout => write!(f, "{} Gateway Timeout", (*self as usize)),
            Self::HTTPVersionNotSupported => {
                write!(f, "{} HTTP Version Not Supported", (*self as usize))
            }
        }
    }
}
