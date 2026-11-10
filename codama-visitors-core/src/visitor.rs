use codama_nodes::NodeRef;

/// Visits a node of a Codama tree and returns a value of type `T` for it.
///
/// A visitor handles one node at a time and decides whether, and how, to visit its children.
/// It visits a child through `recurse`, which goes back through the whole visitor it belongs to,
/// including the adapters that wrap it, such as [`VisitorExt::extend`](crate::VisitorExt::extend) or
/// [`VisitorExt::record_node_stack`](crate::VisitorExt::record_node_stack). This is how a wrapped visitor keeps its behaviour on every
/// node of the tree, like `self` in the visitors of the JavaScript library.
///
/// The lifetime `'a` is the lifetime of the tree being visited, which lets a visitor keep
/// references to its nodes, for example in a [`NodeStack`](crate::NodeStack).
///
/// Visitors take `&self`, so a visitor that keeps state between nodes stores it in a `Cell` or a
/// `RefCell`, borrowed for as short as possible and never across a call to `recurse`.
///
/// Start a visit with the [`visit`] function rather than this method, as it provides `recurse`.
///
/// ```
/// use codama_nodes::{NodeRef, ProgramNode, RootNode};
/// use codama_visitors_core::{visit, Visitor};
///
/// /// Counts every node of a tree.
/// struct CountVisitor;
///
/// impl<'a> Visitor<'a, usize> for CountVisitor {
///     fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> usize) -> usize {
///         1 + node.children().into_iter().map(recurse).sum::<usize>()
///     }
/// }
///
/// let root = RootNode::new(ProgramNode::default());
/// assert_eq!(visit(&CountVisitor, &root), 2);
/// ```
pub trait Visitor<'a, T> {
    /// Visits `node`, where `recurse` visits a child node with the whole visitor.
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T;
}

/// Visits `node` with `visitor`, which visits the children of `node` with itself.
pub fn visit<'a, T, V>(visitor: &V, node: impl Into<NodeRef<'a>>) -> T
where
    V: Visitor<'a, T> + ?Sized,
{
    visit_node(visitor, node.into())
}

fn visit_node<'a, T, V>(visitor: &V, node: NodeRef<'a>) -> T
where
    V: Visitor<'a, T> + ?Sized,
{
    visitor.visit(node, &|child| visit_node(visitor, child))
}

impl<'a, T, V> Visitor<'a, T> for &V
where
    V: Visitor<'a, T> + ?Sized,
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        (**self).visit(node, recurse)
    }
}

impl<'a, T, V> Visitor<'a, T> for Box<V>
where
    V: Visitor<'a, T> + ?Sized,
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        (**self).visit(node, recurse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{HasKind, IntegerTypeNode, StructFieldTypeNode, StructTypeNode, U8};
    use std::cell::RefCell;

    /// Records the kind of every node it visits, then visits its children.
    #[derive(Default)]
    struct KindsVisitor(RefCell<Vec<&'static str>>);

    impl<'a> Visitor<'a, ()> for KindsVisitor {
        fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>)) {
            self.0.borrow_mut().push(node.kind());
            node.children().into_iter().for_each(recurse);
        }
    }

    fn r#struct() -> StructTypeNode {
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            IntegerTypeNode::le(U8),
        )])
    }

    #[test]
    fn visit_recurses_through_the_visitor() {
        let visitor = KindsVisitor::default();
        visit(&visitor, &r#struct());
        assert_eq!(
            *visitor.0.borrow(),
            ["structTypeNode", "structFieldTypeNode", "integerTypeNode"]
        );
    }

    #[test]
    fn visit_through_a_reference_or_a_box() {
        let r#struct = r#struct();
        let visitor = KindsVisitor::default();
        visit(&&visitor, &r#struct);
        let boxed: Box<dyn Visitor<()>> = Box::new(&visitor);
        visit(&boxed, &r#struct);
        assert_eq!(visitor.0.borrow().len(), 6);
    }
}
