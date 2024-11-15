pub use BytesEncoding::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BytesEncoding {
    Base16,
    Base58,
    Base64,
    Utf8,
}
