//! A library of ready-made visitors that transform a Codama IDL.
//!
//! This crate is the Rust counterpart of the TypeScript `@codama/visitors`
//! package. It builds on the framework in [`codama_visitors_core`] (the
//! [`TransformVisitor`] trait and its `fold_*` helpers) and adds concrete,
//! task-specific transforms.
//!
//! Everything in `codama-visitors-core` is re-exported here, so depending on
//! `codama-visitors` is enough to get both the framework and the utility
//! visitors — mirroring how the upstream `@codama/visitors` re-exports
//! `@codama/visitors-core`.
//!
//! [`TransformVisitor`]: codama_visitors_core::TransformVisitor

pub use codama_visitors_core::*;

mod set_account_discriminator_from_field;
mod set_instruction_account_default_values;
mod set_instruction_discriminators;
mod set_number_wrappers;
mod transform_u8_arrays_to_bytes;
mod update_errors;

pub use set_account_discriminator_from_field::*;
pub use set_instruction_account_default_values::*;
pub use set_instruction_discriminators::*;
pub use set_number_wrappers::*;
pub use transform_u8_arrays_to_bytes::*;
pub use update_errors::*;
