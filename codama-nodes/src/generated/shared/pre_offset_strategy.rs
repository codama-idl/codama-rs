use serde::{Deserialize, Serialize};

/// How a pre-offset modifier interprets its offset value before serialising the wrapped type.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PreOffsetStrategy {
    /// Move the cursor to the absolute byte position given by the offset.
    Absolute,
    /// Pad with zero bytes from the current cursor up to the offset bytes ahead.
    Padded,
    /// Advance the cursor by the offset bytes relative to its current position.
    Relative,
}
