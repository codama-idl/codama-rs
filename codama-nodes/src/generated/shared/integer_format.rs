use serde::{Deserialize, Serialize};

/// The wire format of an integer serialization.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IntegerFormat {
    /// Signed 8-bit integer.
    I8,
    /// Signed 16-bit integer.
    I16,
    /// Signed 32-bit integer.
    I32,
    /// Signed 64-bit integer.
    I64,
    /// Signed 128-bit integer.
    I128,
    /// Solana compact-u16 encoding: a variable-length unsigned integer occupying 1 to 3 bytes. Values up to `0x7f` are stored as-is in a single byte; above that, the top bit of each byte is set and the remaining value continues in the next byte, with the third byte — when needed — using all 8 bits.
    /// As a byte-oriented format, it ignores the `endian` attribute.
    ShortU16,
    /// Unsigned 8-bit integer.
    U8,
    /// Unsigned 16-bit integer.
    U16,
    /// Unsigned 32-bit integer.
    U32,
    /// Unsigned 64-bit integer.
    U64,
    /// Unsigned 128-bit integer.
    U128,
}
