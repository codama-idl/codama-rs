use serde::{Deserialize, Serialize};

/// How a post-offset transform interprets its offset value after serialising the transformed type.
/// See `postOffsetTransformNode` for an illustrated walkthrough of each strategy.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostOffsetStrategy {
    /// Move the cursor to the absolute byte position given by the offset; a negative offset counts backwards from the end of the buffer.
    Absolute,
    /// Move the cursor like `relative` while growing the buffer by the offset amount; a negative offset moves the cursor backwards and shrinks the buffer.
    Padded,
    /// Move the cursor by the offset bytes relative to the pre-offset — where the transformed type started — rather than where it ended; a negative offset moves it to the left of that position.
    PreOffset,
    /// Advance the cursor by the offset bytes relative to its current position; a negative offset moves it backwards.
    Relative,
}
