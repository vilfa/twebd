pub mod buffer;
pub mod parse;

pub trait Parse<T, V, E: Sized> {
    fn parse(v: V) -> std::result::Result<T, E>;
}

pub trait ToBuf {
    fn to_buf(&self) -> Vec<u8>;
}
