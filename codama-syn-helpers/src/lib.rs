//! Extensions of `syn` used to parse Codama attributes.
//!
//! [`Meta`] parses the arguments of an attribute more loosely than `syn::Meta`, so directives can take
//! types and values as arguments, and the traits of [`extensions`] add helpers to `syn` types, such as
//! reading a path as a string or an expression as an integer.

pub mod extensions;

mod meta;
pub use meta::*;
