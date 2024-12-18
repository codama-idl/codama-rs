use codama_errors::{CodamaError, CodamaResult};
use serde::{Deserialize, Serialize};
pub use BytesEncoding::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BytesEncoding {
    Base16,
    Base58,
    Base64,
    Utf8,
}

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
