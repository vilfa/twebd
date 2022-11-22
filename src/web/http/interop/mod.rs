pub mod buffer;
pub mod parse;

pub use buffer::{stringify, tokenize_s, TokenIter};

use crate::web::HttpParseError;

pub trait HttpParser<T, V>
where
    T: Sized,
    V: Sized,
{
    fn parse(v: T) -> std::result::Result<V, HttpParseError>;
}

pub trait ToBuffer {
    fn to_buf(&self) -> Vec<u8>;
}
