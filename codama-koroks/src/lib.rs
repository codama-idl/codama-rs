//! A tree of the items of your crates, each with the node it describes.
//!
//! A korok wraps an item of a crate loaded by `codama-stores`, such as a struct or an enum variant, along
//! with its parsed attributes and the [`Node`](codama_nodes::Node) it currently resolves to, if any.
//! [`RootKorok`] is the root of the tree: the node it ends up with is the IDL. The korok visitors of
//! `codama-korok-visitors` fill and combine these nodes.

mod const_korok;
mod crate_korok;
mod enum_korok;
mod enum_variant_korok;
mod field_korok;
mod file_module_korok;
mod impl_item_korok;
mod impl_korok;
mod item_korok;
mod korok;
mod module_korok;
mod root_korok;
mod struct_korok;
mod type_alias_korok;
mod unsupported_impl_item_korok;
mod unsupported_item_korok;

pub use const_korok::*;
pub use crate_korok::*;
pub use enum_korok::*;
pub use enum_variant_korok::*;
pub use field_korok::*;
pub use file_module_korok::*;
pub use impl_item_korok::*;
pub use impl_korok::*;
pub use item_korok::*;
pub use korok::*;
pub use module_korok::*;
pub use root_korok::*;
pub use struct_korok::*;
pub use type_alias_korok::*;
pub use unsupported_impl_item_korok::*;
pub use unsupported_item_korok::*;
