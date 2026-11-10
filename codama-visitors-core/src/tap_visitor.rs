use crate::Visitor;
use codama_nodes::NodeRef;

/// Wraps a visitor with a function that runs on every node before the wrapped visitor visits it.
///
/// Created by [`VisitorExt::tap`](crate::VisitorExt::tap). The function cannot change the visit,
/// only observe it, and pattern-matches the node to act on some kinds only.
#[derive(Debug, Clone, Copy)]
pub struct TapVisitor<V, F> {
    visitor: V,
    tap: F,
}

impl<V, F> TapVisitor<V, F> {
    /// Wraps `visitor` with `tap`. Prefer [`VisitorExt::tap`](crate::VisitorExt::tap).
    pub fn new<'a, T>(visitor: V, tap: F) -> Self
    where
        V: Visitor<'a, T>,
        F: Fn(NodeRef<'a>),
    {
        Self { visitor, tap }
    }
}

impl<'a, T, V, F> Visitor<'a, T> for TapVisitor<V, F>
where
    V: Visitor<'a, T>,
    F: Fn(NodeRef<'a>),
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        (self.tap)(node);
        self.visitor.visit(node, recurse)
    }
}

#[cfg(test)]
mod tests {
    use crate::{visit, IdentityVisitor, VisitorExt};
    use codama_nodes::{HasKind, IntegerTypeNode, Node, NodeRef, StructFieldTypeNode, U8};
    use std::cell::RefCell;

    #[test]
    fn taps_every_node_before_visiting_it() {
        let kinds = RefCell::new(vec![]);
        let visitor = IdentityVisitor.tap(|node: NodeRef| kinds.borrow_mut().push(node.kind()));
        let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
        assert_eq!(visit(&visitor, &field).unwrap(), Some(Node::from(field)));
        assert_eq!(*kinds.borrow(), ["structFieldTypeNode", "integerTypeNode"]);
    }
}
