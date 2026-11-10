use crate::{visit, Visitor};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{HasKind, Node, NodeRef};

/// Makes a visitor based on [`IdentityVisitor`](crate::IdentityVisitor) return the visited node,
/// with an error if it deletes it instead.
///
/// Created by [`VisitorExt::non_null`](crate::VisitorExt::non_null). Use it when the visit must
/// return a node, for example to transform a whole tree. Like `nonNullableIdentityVisitor` in the
/// JavaScript library, only the node that the visit starts from must remain: its descendants can
/// still be deleted. Like [`VisitorExt::map`](crate::VisitorExt::map), it applies to that node only,
/// so wrap a visitor with it last.
#[derive(Debug, Clone, Copy)]
pub struct NonNullableVisitor<V> {
    visitor: V,
}

impl<V> NonNullableVisitor<V> {
    /// Wraps `visitor`. Prefer [`VisitorExt::non_null`](crate::VisitorExt::non_null).
    pub fn new<'a>(visitor: V) -> Self
    where
        V: Visitor<'a, CodamaResult<Option<Node>>>,
    {
        Self { visitor }
    }
}

impl<'a, V> Visitor<'a, CodamaResult<Node>> for NonNullableVisitor<V>
where
    V: Visitor<'a, CodamaResult<Option<Node>>>,
{
    /// Visits `node` with the wrapped visitor alone, so `recurse` is not used.
    fn visit(
        &self,
        node: NodeRef<'a>,
        _recurse: &dyn Fn(NodeRef<'a>) -> CodamaResult<Node>,
    ) -> CodamaResult<Node> {
        visit(&self.visitor, node)?.ok_or_else(|| CodamaError::DeletedNode {
            kind: node.kind().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{visit, IdentityVisitor, VisitorExt};
    use codama_nodes::{
        IntegerTypeNode, Node, NodeRef, StringTypeNode, StructFieldTypeNode, StructTypeNode, U8,
    };

    #[test]
    fn returns_the_visited_node() {
        let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
        assert_eq!(
            visit(&IdentityVisitor.non_null(), &field).unwrap(),
            Node::from(field)
        );
    }

    #[test]
    fn still_deletes_descendants() {
        let visitor = IdentityVisitor
            .extend(|node, scope| match node {
                NodeRef::IntegerType(_) => Ok(None),
                node => scope.next(node),
            })
            .non_null();
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8)),
            StructFieldTypeNode::new("label", StringTypeNode::utf8()),
        ]);
        let expected = StructTypeNode::new(vec![StructFieldTypeNode::new(
            "label",
            StringTypeNode::utf8(),
        )]);
        assert_eq!(visit(&visitor, &r#struct).unwrap(), Node::from(expected));
    }

    #[test]
    fn fails_when_the_visited_node_is_deleted() {
        let visitor = IdentityVisitor
            .extend(|node, scope| match node {
                NodeRef::IntegerType(_) => Ok(None),
                node => scope.next(node),
            })
            .non_null();
        let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
        let error = visit(&visitor, &field).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Expected a node but the `structFieldTypeNode` was deleted"
        );
    }
}
