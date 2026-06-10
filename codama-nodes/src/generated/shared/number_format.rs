use serde::{Deserialize, Serialize};

/// The wire format of a numeric serialization.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormat {
    /// IEEE-754 32-bit floating point.
    F32,
    /// IEEE-754 64-bit floating point.
    F64,
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
    /// Solana compact-u16 encoding: a variable-length unsigned integer occupying 1 to 3 bytes.
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
