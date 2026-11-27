//! Parse Codama attributes into directives.
//!
//! [`Attributes`] gathers the attributes of a Rust item, and each `#[codama(...)]` attribute parses into a
//! [`CodamaDirective`], such as a [`TypeDirective`] or an [`AccountDirective`], which the korok visitors
//! turn into nodes. A directive that a plugin resolves itself, such as `type = my_plugin::custom`, parses
//! into a [`ResolvableDirective`].

mod utils;

mod attribute;
mod attribute_context;
mod attributes;
mod codama_attribute;
mod codama_directives;
mod codama_program_macro;
mod derive_attribute;
mod repr_attribute;
mod try_from_filter;
mod unsupported_attribute;

pub use attribute::*;
pub use attribute_context::*;
pub use attributes::*;
pub use codama_attribute::*;
pub use codama_directives::*;
pub use codama_program_macro::*;
pub use derive_attribute::*;
pub use repr_attribute::*;
pub use try_from_filter::*;
pub use unsupported_attribute::*;
