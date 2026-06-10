use serde::{Deserialize, Serialize};

/// How a post-offset modifier interprets its offset value after serialising the wrapped type.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostOffsetStrategy {
    /// Move the cursor to the absolute byte position given by the offset.
    Absolute,
    /// Pad with zero bytes from the current cursor up to the offset bytes ahead.
    Padded,
    /// Restore the cursor to where it was before the wrapped type ran (cancelling its pre-offset).
    PreOffset,
    /// Advance the cursor by the offset bytes relative to its current position.
    Relative,
}
