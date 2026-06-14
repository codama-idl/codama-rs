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

mod add_pdas;
mod deduplicate_identical_defined_types;
mod flatten_instruction_data_arguments;
mod flatten_struct;
mod get_defined_type_histogram;
mod set_account_discriminator_from_field;
mod set_fixed_account_sizes;
mod set_instruction_account_default_values;
mod set_instruction_discriminators;
mod set_number_wrappers;
mod set_struct_default_values;
mod transform_u8_arrays_to_bytes;
mod update_defined_types;
mod update_errors;
mod update_programs;

pub use add_pdas::*;
pub use deduplicate_identical_defined_types::*;
pub use flatten_instruction_data_arguments::*;
pub use flatten_struct::*;
pub use get_defined_type_histogram::*;
pub use set_account_discriminator_from_field::*;
pub use set_fixed_account_sizes::*;
pub use set_instruction_account_default_values::*;
pub use set_instruction_discriminators::*;
pub use set_number_wrappers::*;
pub use set_struct_default_values::*;
pub use transform_u8_arrays_to_bytes::*;
pub use update_defined_types::*;
pub use update_errors::*;
pub use update_programs::*;
