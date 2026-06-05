use serde::{Deserialize, Serialize};

/// The toolchain that originally generated a program description.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgramOrigin {
    /// The program was originally described by an Anchor IDL.
    Anchor,
    /// The program was originally described by a Shank IDL.
    Shank,
}
