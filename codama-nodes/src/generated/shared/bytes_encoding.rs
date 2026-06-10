use serde::{Deserialize, Serialize};

/// How a string of bytes is encoded for transport.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BytesEncoding {
    /// Hexadecimal encoding (two characters per byte).
    Base16,
    /// Base58 encoding, the standard for Solana addresses.
    Base58,
    /// Base64 encoding (RFC 4648).
    Base64,
    /// UTF-8 text encoding.
    Utf8,
}
