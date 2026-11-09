use crate::Visitor;
use codama_errors::CodamaResult;
use codama_nodes::{Node, NodeRef};

/// Rebuilds the visited tree node by node, as a base for visitors that transform a tree.
///
/// Each node is rebuilt from the results of visiting its children, so a visitor built on top of it,
/// for example with [`VisitorExt::extend`](crate::VisitorExt::extend), can replace or delete some
/// nodes and leave the rest of the tree untouched. Returning `None` deletes a node:
///
/// - in a list, the item is dropped;
/// - in an optional attribute, the attribute becomes empty;
/// - in a required attribute, the parent node is deleted too, and so on up the tree.
///
/// A node returned for an attribute that cannot hold its kind is an error.
/// Use [`VisitorExt::non_null`](crate::VisitorExt::non_null) when the visited node itself must remain.
///
/// ```
/// use codama_nodes::{IntegerTypeNode, Node, NodeRef, StringTypeNode, StructFieldTypeNode, U8};
/// use codama_visitors_core::{visit, IdentityVisitor, VisitorExt};
///
/// // Turns every integer into a string.
/// let visitor = IdentityVisitor.extend(|node, scope| match node {
///     NodeRef::IntegerType(_) => Ok(Some(StringTypeNode::utf8().into())),
///     node => scope.next(node),
/// });
///
/// let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
/// let expected = StructFieldTypeNode::new("amount", StringTypeNode::utf8());
/// assert_eq!(visit(&visitor, &field).unwrap(), Some(Node::from(expected)));
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityVisitor;

impl<'a> Visitor<'a, CodamaResult<Option<Node>>> for IdentityVisitor {
    fn visit(
        &self,
        node: NodeRef<'a>,
        recurse: &dyn Fn(NodeRef<'a>) -> CodamaResult<Option<Node>>,
    ) -> CodamaResult<Option<Node>> {
        node.map_children(recurse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{visit, VisitorExt};
    use codama_nodes::{
        DefinedTypeNode, IntegerTypeNode, ProgramNode, RootNode, StringTypeNode,
        StructFieldTypeNode, StructTypeNode, U64, U8,
    };

    fn root() -> RootNode {
        RootNode::new(ProgramNode {
            defined_types: vec![DefinedTypeNode::new(
                "config",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("authority", StringTypeNode::utf8()),
                    StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                ]),
            )],
            ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        })
    }

    #[test]
    fn rebuilds_the_same_tree() {
        let root = root();
        assert_eq!(
            visit(&IdentityVisitor, &root).unwrap(),
            Some(Node::from(root))
        );
    }

    #[test]
    fn replaces_nodes_anywhere_in_the_tree() {
        let visitor = IdentityVisitor.extend(|node, scope| match node {
            NodeRef::IntegerType(_) => Ok(Some(IntegerTypeNode::le(U8).into())),
            node => scope.next(node),
        });
        let Some(Node::Root(root)) = visit(&visitor, &root()).unwrap() else {
            panic!("expected a root node");
        };
        let r#type = &*root.program.defined_types[0].r#type;
        assert_eq!(
            r#type,
            &StructTypeNode::new(vec![
                StructFieldTypeNode::new("authority", StringTypeNode::utf8()),
                StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8)),
            ])
            .into()
        );
    }

    #[test]
    fn deleting_a_required_child_deletes_its_parents_up_to_a_list() {
        // The integer is the required type of the `amount` field, so the field is deleted too,
        // and dropped from the list of fields.
        let visitor = IdentityVisitor.extend(|node, scope| match node {
            NodeRef::IntegerType(_) => Ok(None),
            node => scope.next(node),
        });
        let Some(Node::Root(root)) = visit(&visitor, &root()).unwrap() else {
            panic!("expected a root node");
        };
        let r#type = &*root.program.defined_types[0].r#type;
        assert_eq!(
            r#type,
            &StructTypeNode::new(vec![StructFieldTypeNode::new(
                "authority",
                StringTypeNode::utf8()
            )])
            .into()
        );
    }

    #[test]
    fn deleting_the_visited_node_returns_none() {
        let visitor = IdentityVisitor.extend(|node, scope| match node {
            NodeRef::Program(_) => Ok(None),
            node => scope.next(node),
        });
        assert_eq!(visit(&visitor, &root()).unwrap(), None);
    }
}
