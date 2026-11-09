//! Helpers for the generated `NodeRef::map_children`, one per shape of child slot.
//!
//! Each helper maps the children of one slot with `f` and converts the results back into the type of the slot.
//! A child that `f` deletes (`None`) is dropped from a list and empties an optional slot. For a required slot,
//! the helper returns `None` so the generated code deletes the parent too.

use crate::{Docs, Node, NodeRef, Text, TextNode};
use codama_errors::{CodamaError, CodamaResult};

/// The function that maps one child node, as `NodeRef::map_children` receives it.
pub(crate) type MapChild<'f, 'a> = dyn FnMut(NodeRef<'a>) -> CodamaResult<Option<Node>> + 'f;

/// Maps the child of a required slot, or `None` when `f` deletes it.
pub(crate) fn map_required<'a, T>(
    child: NodeRef<'a>,
    f: &mut MapChild<'_, 'a>,
) -> CodamaResult<Option<T>>
where
    T: TryFrom<Node, Error = CodamaError>,
{
    f(child)?.map(T::try_from).transpose()
}

/// Maps the child of an optional slot. The slot stays empty when it is empty or when `f` deletes its child.
pub(crate) fn map_optional<'a, T>(
    child: Option<NodeRef<'a>>,
    f: &mut MapChild<'_, 'a>,
) -> CodamaResult<Option<T>>
where
    T: TryFrom<Node, Error = CodamaError>,
{
    match child {
        Some(child) => map_required(child, f),
        None => Ok(None),
    }
}

/// Maps every item of a list slot, dropping the items that `f` deletes.
pub(crate) fn map_array<'a, T, C>(
    children: &'a [C],
    f: &mut MapChild<'_, 'a>,
) -> CodamaResult<Vec<T>>
where
    T: TryFrom<Node, Error = CodamaError>,
    &'a C: Into<NodeRef<'a>>,
{
    children
        .iter()
        .filter_map(|child| map_required(child.into(), f).transpose())
        .collect()
}

/// Maps a required text slot. Plain text has no child node, so only a `textNode` goes through `f`.
pub(crate) fn map_text<'a>(text: &'a Text, f: &mut MapChild<'_, 'a>) -> CodamaResult<Option<Text>> {
    match text {
        Text::String(content) => Ok(Some(Text::String(content.clone()))),
        Text::Node(node) => Ok(map_required::<TextNode>(node.into(), f)?.map(Text::Node)),
    }
}

/// Maps an optional text slot. It stays empty when it is empty or when `f` deletes its `textNode`.
pub(crate) fn map_optional_text<'a>(
    text: Option<&'a Text>,
    f: &mut MapChild<'_, 'a>,
) -> CodamaResult<Option<Text>> {
    match text {
        Some(text) => map_text(text, f),
        None => Ok(None),
    }
}

/// Maps a docs slot. The docs become empty when `f` deletes their `textNode`.
pub(crate) fn map_docs<'a>(docs: &'a Docs, f: &mut MapChild<'_, 'a>) -> CodamaResult<Docs> {
    Ok(map_optional_text(docs.as_ref(), f)?
        .map(Docs::from)
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use crate::{
        DefinedTypeLinkNode, Docs, ErrorNode, HasKind, Identifier, IntegerTypeNode,
        IntegerValueNode, Node, NodeRef, PluginNode, ProgramNode, StringTypeNode,
        StructFieldDisplayNode, StructFieldTypeNode, StructTypeNode, Text, TextNode, U8,
    };
    use codama_errors::CodamaResult;

    fn text_node(content: &str) -> TextNode {
        TextNode {
            content: content.to_string(),
            plugins: vec![],
        }
    }

    fn plugin() -> PluginNode {
        PluginNode::new("anchor".parse::<crate::Namespace>().unwrap())
    }

    /// A field with a child in every kind of slot: docs, a required union, an optional union,
    /// an optional node and a list.
    fn field() -> StructFieldTypeNode {
        StructFieldTypeNode {
            docs: Docs::from(Text::Node(text_node("The amount."))),
            default_value: Box::new(Some(IntegerValueNode::new(42).into())),
            display: Some(StructFieldDisplayNode::new("Amount")),
            plugins: vec![plugin()],
            ..StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8))
        }
    }

    fn keep(child: NodeRef) -> CodamaResult<Option<Node>> {
        Ok(Some(child.to_node()))
    }

    fn kinds(children: Vec<NodeRef>) -> Vec<&'static str> {
        children.into_iter().map(|child| child.kind()).collect()
    }

    #[test]
    fn children_in_spec_order() {
        let field = field();
        assert_eq!(
            kinds(NodeRef::from(&field).children()),
            [
                "textNode",
                "integerTypeNode",
                "integerValueNode",
                "structFieldDisplayNode",
                "pluginNode"
            ]
        );
    }

    #[test]
    fn children_skip_plain_text_and_empty_slots() {
        let field = StructFieldTypeNode {
            docs: Docs::from(Text::from("The amount.")),
            ..StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8))
        };
        assert_eq!(kinds(NodeRef::from(&field).children()), ["integerTypeNode"]);
    }

    #[test]
    fn map_children_rebuilds_the_same_node() {
        let field = field();
        let rebuilt = NodeRef::from(&field).map_children(keep).unwrap();
        assert_eq!(rebuilt, Some(Node::from(field)));
    }

    #[test]
    fn map_children_maps_children_in_spec_order() {
        let field = field();
        let mut visited = vec![];
        NodeRef::from(&field)
            .map_children(|child| {
                visited.push(child.kind());
                keep(child)
            })
            .unwrap();
        assert_eq!(visited, kinds(NodeRef::from(&field).children()));
    }

    #[test]
    fn map_children_replaces_a_child() {
        let field = field();
        let rebuilt = NodeRef::from(&field)
            .map_children(|child| match child {
                NodeRef::IntegerType(_) => Ok(Some(StringTypeNode::utf8().into())),
                child => keep(child),
            })
            .unwrap();
        let Some(Node::StructFieldType(rebuilt)) = rebuilt else {
            panic!("expected a struct field");
        };
        assert_eq!(*rebuilt.r#type, StringTypeNode::utf8().into());
    }

    #[test]
    fn map_children_deletes_optional_children_and_list_items() {
        let field = field();
        let rebuilt = NodeRef::from(&field)
            .map_children(|child| match child {
                NodeRef::Text(_) | NodeRef::IntegerValue(_) | NodeRef::Plugin(_) => Ok(None),
                NodeRef::StructFieldDisplay(_) => Ok(None),
                child => keep(child),
            })
            .unwrap();
        assert_eq!(
            rebuilt,
            Some(StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8)).into())
        );
    }

    #[test]
    fn map_children_deletes_the_node_when_a_required_child_is_deleted() {
        let field = field();
        let rebuilt = NodeRef::from(&field)
            .map_children(|child| match child {
                NodeRef::IntegerType(_) => Ok(None),
                child => keep(child),
            })
            .unwrap();
        assert_eq!(rebuilt, None);

        let error = ErrorNode::new("myError", 42, Text::Node(text_node("Oops")));
        let rebuilt = NodeRef::from(&error)
            .map_children(|child| match child {
                NodeRef::Text(_) => Ok(None),
                child => keep(child),
            })
            .unwrap();
        assert_eq!(rebuilt, None);
    }

    #[test]
    fn map_children_drops_struct_fields_from_a_list() {
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("a", IntegerTypeNode::le(U8)),
            StructFieldTypeNode::new("b", IntegerTypeNode::le(U8)),
        ]);
        let rebuilt = NodeRef::from(&r#struct)
            .map_children(|child| match child {
                NodeRef::StructFieldType(field) if field.identifier.as_str() == "a" => Ok(None),
                child => keep(child),
            })
            .unwrap();
        assert_eq!(
            rebuilt,
            Some(
                StructTypeNode::new(vec![StructFieldTypeNode::new("b", IntegerTypeNode::le(U8))])
                    .into()
            )
        );
    }

    #[test]
    fn map_children_rejects_a_child_of_the_wrong_kind() {
        let field = field();
        let error = NodeRef::from(&field)
            .map_children(|child| match child {
                NodeRef::IntegerType(_) => Ok(Some(plugin().into())),
                child => keep(child),
            })
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "Could not convert node from `pluginNode` into `TypeNode`"
        );
    }

    #[test]
    fn identifier() {
        let program = ProgramNode {
            identifier: "token".into(),
            ..ProgramNode::default()
        };
        assert_eq!(
            NodeRef::from(&program).identifier(),
            Some(&Identifier::from("token"))
        );
        let link = DefinedTypeLinkNode::new("myType");
        assert_eq!(
            NodeRef::from(&link).identifier(),
            Some(&Identifier::from("myType"))
        );
        assert_eq!(NodeRef::from(&StringTypeNode::utf8()).identifier(), None);
    }

    #[test]
    fn node_struct_try_from_node_ref() {
        let program = ProgramNode::default();
        let node_ref = NodeRef::from(&program);
        assert!(std::ptr::eq(
            <&ProgramNode>::try_from(node_ref).unwrap(),
            &program
        ));
        let error = <&StringTypeNode>::try_from(node_ref).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Could not convert node from `programNode` into `stringTypeNode`"
        );
    }
}
