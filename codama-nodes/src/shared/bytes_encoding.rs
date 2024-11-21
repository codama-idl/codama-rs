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
