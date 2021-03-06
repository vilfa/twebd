pub mod buffer;
pub mod parse;

use crate::web::http::consts;
use crate::web::HttpParseError;

pub trait Parse<T, V, E>
where
    T: Sized,
    V: Sized,
    E: Sized,
{
    fn parse(v: T) -> std::result::Result<V, E>;
}

pub trait ToBuf {
    fn to_buf(&self) -> Vec<u8>;
}

pub fn buffer_to_string(buf: &[u8]) -> Result<String, HttpParseError> {
    match std::str::from_utf8(buf) {
        Ok(v) => match regex::Regex::new(" +") {
            Ok(r) => {
                let buf = r.replace_all(v.trim(), consts::WSPC).to_string();
                Ok(buf)
            }
            Err(e) => Err(HttpParseError::BufferParseError(format!("`{:?}`", e))),
        },
        Err(e) => Err(HttpParseError::BufferParseError(format!("`{:?}`", e))),
    }
}

pub type TokenIter<'a> = std::vec::IntoIter<&'a str>;

pub fn string_into_tokens(str: &String) -> TokenIter {
    str.split(consts::CRLF)
        .map(|v| v.trim())
        .collect::<Vec<&str>>()
        .into_iter()
}
