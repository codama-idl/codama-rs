use serde::{Deserialize, Serialize};

/// The byte order of a numeric serialization.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Endianness {
    /// Big-endian: the most significant byte is written first.
    Be,
    /// Little-endian: the least significant byte is written first.
    Le,
}
