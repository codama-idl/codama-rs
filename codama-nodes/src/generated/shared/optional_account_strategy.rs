use serde::{Deserialize, Serialize};

/// How an absent optional account is represented when serialising an instruction.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OptionalAccountStrategy {
    /// The account slot is left out of the instruction entirely. Subsequent accounts shift up.
    Omitted,
    /// The account slot is filled with the program ID as a placeholder, preserving positional indices.
    ProgramId,
}
