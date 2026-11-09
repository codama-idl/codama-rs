use crate::Visitor;
use codama_nodes::NodeRef;

/// Visits every node of a tree and merges the values of its children into the value of the node.
///
/// For each node, the visitor first visits its children, then calls `merge` with the node and the
/// values of its children, in the order of the attributes that hold them. A node without children
/// gets an empty list.
///
/// Unlike `mergeVisitor` in the JavaScript library, there is no separate function for leaf nodes:
/// every node of the spec can hold plugins, so every node goes through `merge`.
///
/// ```
/// use codama_nodes::{HasKind, IntegerTypeNode, StructFieldTypeNode, U8};
/// use codama_visitors_core::{visit, MergeVisitor};
///
/// // Lists the kinds of every node, parents first.
/// let visitor = MergeVisitor::new(|node, values: Vec<Vec<&str>>| {
///     std::iter::once(node.kind()).chain(values.into_iter().flatten()).collect()
/// });
///
/// let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
/// assert_eq!(visit(&visitor, &field), ["structFieldTypeNode", "integerTypeNode"]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MergeVisitor<F> {
    merge: F,
}

impl<F> MergeVisitor<F> {
    /// Merges the values of the children of each node with `merge`.
    pub fn new<'a, T>(merge: F) -> Self
    where
        F: Fn(NodeRef<'a>, Vec<T>) -> T,
    {
        Self { merge }
    }
}

impl<'a, T, F> Visitor<'a, T> for MergeVisitor<F>
where
    F: Fn(NodeRef<'a>, Vec<T>) -> T,
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        let values = node.children().into_iter().map(recurse).collect();
        (self.merge)(node, values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visit;
    use codama_nodes::{
        HasKind, IntegerTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U8,
    };

    #[test]
    fn merges_the_values_of_the_children_in_order() {
        let visitor = MergeVisitor::new(|node, values: Vec<String>| match values.is_empty() {
            true => node.kind().to_string(),
            false => format!("{}({})", node.kind(), values.join(", ")),
        });
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("a", IntegerTypeNode::le(U8)),
            StructFieldTypeNode::new("b", IntegerTypeNode::le(U8)),
        ]);
        assert_eq!(
            visit(&visitor, &r#struct),
            "structTypeNode(structFieldTypeNode(integerTypeNode), structFieldTypeNode(integerTypeNode))"
        );
    }

    #[test]
    fn calls_merge_on_nodes_without_children() {
        let visitor = MergeVisitor::new(|_, values: Vec<usize>| 1 + values.iter().sum::<usize>());
        assert_eq!(visit(&visitor, &RootNode::new(ProgramNode::default())), 2);
    }
}
