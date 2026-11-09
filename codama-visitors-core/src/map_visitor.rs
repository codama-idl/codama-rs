use crate::{visit, Visitor};
use codama_nodes::NodeRef;
use std::marker::PhantomData;

/// Maps the value that a visitor returns for the visited node.
///
/// Created by [`VisitorExt::map`](crate::VisitorExt::map). Only the value of the node that the visit
/// starts from is mapped: the wrapped visitor visits the rest of the tree on its own and gets the
/// values of the children it expects. As in the JavaScript library, map a visitor last, after the
/// adapters that should apply to every node.
#[derive(Debug, Clone, Copy)]
pub struct MapVisitor<V, F, T> {
    visitor: V,
    map: F,
    value: PhantomData<fn() -> T>,
}

impl<V, F, T> MapVisitor<V, F, T> {
    /// Maps the value of `visitor` with `map`. Prefer [`VisitorExt::map`](crate::VisitorExt::map).
    pub fn new<'a, U>(visitor: V, map: F) -> Self
    where
        V: Visitor<'a, T>,
        F: Fn(T) -> U,
    {
        Self {
            visitor,
            map,
            value: PhantomData,
        }
    }
}

impl<'a, T, U, V, F> Visitor<'a, U> for MapVisitor<V, F, T>
where
    V: Visitor<'a, T>,
    F: Fn(T) -> U,
{
    /// Visits `node` with the wrapped visitor alone, so `recurse` is not used.
    fn visit(&self, node: NodeRef<'a>, _recurse: &dyn Fn(NodeRef<'a>) -> U) -> U {
        (self.map)(visit(&self.visitor, node))
    }
}

#[cfg(test)]
mod tests {
    use crate::{visit, MergeVisitor, VisitorExt};
    use codama_nodes::{IntegerTypeNode, StructFieldTypeNode, U8};

    #[test]
    fn maps_the_value_of_the_visited_node_only() {
        let visitor = MergeVisitor::new(|_, values: Vec<usize>| 1 + values.iter().sum::<usize>())
            .map(|count| format!("{count} nodes"));
        let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
        assert_eq!(visit(&visitor, &field), "2 nodes");
    }
}
