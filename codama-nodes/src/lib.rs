//! The nodes of the [Codama IDL](https://github.com/codama-idl/spec).
//!
//! Each kind of node of the spec is a struct, such as [`ProgramNode`] or [`StructTypeNode`], and unions
//! group the nodes that can take the same place, such as [`TypeNode`] or [`ValueNode`]. [`Node`] holds a
//! node of any kind and [`NodeRef`] borrows one, so a tree can be walked without cloning it. Nodes
//! serialise to and from the JSON of the spec.
//!
//! ```
//! use codama_nodes::{NodeTrait, ProgramNode, RootNode};
//!
//! let root = RootNode::new(ProgramNode::new("counter", "Counter111111111111111111111111111111111111"));
//! assert!(root.to_json()?.starts_with(r#"{"kind":"rootNode""#));
//! # Ok::<(), codama_errors::CodamaError>(())
//! ```

#![warn(missing_copy_implementations)]

mod account_node;
mod constant_node;
mod contextual_value_nodes;
mod count_nodes;
mod defined_type_node;
mod discriminator_nodes;
mod display_nodes;
mod error_node;
mod event_node;
mod generated;
mod instruction_account_node;
mod instruction_byte_delta_node;
mod instruction_node;
mod instruction_remaining_accounts_node;
mod instruction_status_node;
mod link_nodes;
mod node;
mod node_ref;
mod pda_node;
mod pda_seed_nodes;
mod plugin_node;
mod program_node;
mod provided_node;
mod root_node;
mod shared;
#[cfg(feature = "dev-context-only-utils")]
#[doc(hidden)]
pub mod test_utils;
mod traits;
mod transform_nodes;
mod type_nodes;
mod value_nodes;

pub use display_nodes::*;
pub use generated::*;
pub use shared::*;
pub use traits::*;
pub use type_nodes::*;

// Serde helper function to use with `#[serde(some_thing = "crate::is_default")]`.
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
