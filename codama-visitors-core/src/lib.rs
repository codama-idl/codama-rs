//! Visit and transform the nodes of a Codama tree, like `@codama/visitors-core` in the JavaScript library.
//!
//! A [`Visitor`] returns a value for each node it visits. The base visitors walk the whole tree:
//!
//! - [`IdentityVisitor`] rebuilds the tree, to transform it;
//! - [`MergeVisitor`] merges the values of the children of each node, to collect data from a tree;
//! - [`VoidVisitor`] returns nothing, to run side effects.
//!
//! The adapters of [`VisitorExt`] wrap a visitor into a new one, for example to handle some kinds of
//! nodes with [`VisitorExt::extend`], or to record the path of the visited node into a [`NodeStack`].
//! A [`LinkableDictionary`] resolves link nodes into the nodes they point to.
//!
//! ```
//! use codama_nodes::{DefinedTypeNode, NodeRef, ProgramNode, RootNode, StringTypeNode};
//! use codama_visitors_core::{visit, MergeVisitor, NodeStack, VisitorExt};
//!
//! let root = RootNode::new(ProgramNode {
//!     defined_types: vec![DefinedTypeNode::new("label", StringTypeNode::utf8())],
//!     ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
//! });
//!
//! // Lists the path of every string type.
//! let stack = NodeStack::default();
//! let visitor = MergeVisitor::new(|_, values: Vec<Vec<String>>| values.concat())
//!     .extend(|node, scope| match node {
//!         NodeRef::StringType(_) => vec![stack.to_string()],
//!         node => scope.next(node),
//!     })
//!     .record_node_stack(&stack);
//!
//! assert_eq!(
//!     visit(&visitor, &root),
//!     ["[rootNode] > [programNode]token > [definedTypeNode]label > [stringTypeNode]"]
//! );
//! ```

mod extend_visitor;
mod identity_visitor;
mod linkable_dictionary;
mod map_visitor;
mod merge_visitor;
mod node_path;
mod node_stack;
mod non_nullable_visitor;
mod record_linkables_visitor;
mod record_node_stack_visitor;
mod tap_visitor;
mod visitor;
mod visitor_ext;
mod void_visitor;

pub use extend_visitor::*;
pub use identity_visitor::*;
pub use linkable_dictionary::*;
pub use map_visitor::*;
pub use merge_visitor::*;
pub use node_path::*;
pub use node_stack::*;
pub use non_nullable_visitor::*;
pub use record_linkables_visitor::*;
pub use record_node_stack_visitor::*;
pub use tap_visitor::*;
pub use visitor::*;
pub use visitor_ext::*;
pub use void_visitor::*;
