use serde::{Deserialize, Serialize};

/// The wire format of a floating-point serialization.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FloatFormat {
    /// IEEE-754 32-bit floating point.
    F32,
    /// IEEE-754 64-bit floating point.
    F64,
}
