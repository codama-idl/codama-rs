//! Test helpers for the Codama crates, compiled with the `dev-context-only-utils` feature.
//! The feature stands in for `cfg(test)`, which the tests of other crates do not set for this crate.
//! Turn it on only in `[dev-dependencies]`.

use crate::{Identifier, InstructionNode, Path, StructFieldTypeNode, TypeNode};

/// Identifier from a valid literal such as `id("amount")`. Panics on an invalid identifier.
pub fn id(value: &str) -> Identifier {
    value.parse().unwrap()
}

/// Path from a valid literal such as `path("header.kind")`. Panics on an invalid path.
pub fn path(value: &str) -> Path {
    value.parse().unwrap()
}

/// Fields of the instruction data, or none when the data is not a struct.
pub fn get_data_fields(instruction: &InstructionNode) -> &[StructFieldTypeNode] {
    match &*instruction.data {
        Some(TypeNode::Struct(data)) => &data.fields,
        _ => &[],
    }
}
