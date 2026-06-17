//! Core visitor framework for traversing and transforming a Codama IDL.
//!
//! This crate is the Rust counterpart of the TypeScript `@codama/visitors-core`
//! package. Where `codama-korok-visitors` walks the *Rust source* (the Korok
//! tree) to *build* an IDL, the visitors here operate on an already-built IDL —
//! a tree of [`codama_nodes::Node`]s rooted at a
//! [`codama_nodes::RootNode`] — to *transform* it.
//!
//! The design mirrors [`syn::fold`]: a [`TransformVisitor`] is a trait whose
//! per-node-kind methods consume a node and return a (possibly rewritten) node
//! of the same kind. Every method has a default body that recurses into the
//! node's children via the free `fold_*` functions, so an implementor only
//! overrides the kinds it cares about and the rest of the tree is rebuilt
//! untouched. To recurse from inside an override, call the matching `fold_*`
//! function (the equivalent of the TS `next` hook).
//!
//! ```ignore
//! use codama_nodes::{NumberTypeNode, Endianness};
//! use codama_visitors_core::TransformVisitor;
//!
//! // A transform that forces every number to big-endian.
//! struct ForceBigEndian;
//! impl TransformVisitor for ForceBigEndian {
//!     fn visit_number_type(&mut self, mut node: NumberTypeNode) -> NumberTypeNode {
//!         node.endian = Endianness::Be;
//!         node
//!     }
//! }
//!
//! let new_root = ForceBigEndian.visit_root(root);
//! ```
//!
//! [`syn::fold`]: https://docs.rs/syn/latest/syn/fold/index.html

mod transform_visitor;

pub use transform_visitor::*;
