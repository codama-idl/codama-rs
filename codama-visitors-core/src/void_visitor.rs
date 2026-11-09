use crate::Visitor;
use codama_nodes::NodeRef;

/// Visits every node of a tree and returns nothing.
///
/// Its use is to run the side effects of the visitors that wrap it on every node, for example with
/// [`VisitorExt::tap`](crate::VisitorExt::tap).
///
/// ```
/// use codama_nodes::{IntegerTypeNode, NodeRef, StructFieldTypeNode, U8};
/// use codama_visitors_core::{visit, VisitorExt, VoidVisitor};
/// use std::cell::Cell;
///
/// // Counts the integers of a tree.
/// let count = Cell::new(0);
/// let visitor = VoidVisitor.tap(|node| {
///     if let NodeRef::IntegerType(_) = node {
///         count.set(count.get() + 1);
///     }
/// });
///
/// visit(&visitor, &StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8)));
/// assert_eq!(count.get(), 1);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct VoidVisitor;

impl<'a> Visitor<'a, ()> for VoidVisitor {
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>)) {
        node.children().into_iter().for_each(recurse);
    }
}
