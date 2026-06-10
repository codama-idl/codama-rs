use crate::BytesEncoding;
use codama_errors::{CodamaError, CodamaResult};
pub use BytesEncoding::*;

impl TryFrom<String> for BytesEncoding {
    type Error = CodamaError;

    fn try_from(value: String) -> CodamaResult<Self> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for BytesEncoding {
    type Error = CodamaError;

    fn try_from(value: &str) -> CodamaResult<Self> {
        match value {
            "base16" => Ok(Base16),
            "base58" => Ok(Base58),
            "base64" => Ok(Base64),
            "utf8" => Ok(Utf8),
            _ => Err(CodamaError::InvalidBytesEncoding(value.to_string())),
        }
    }
}
