use crate::{CamelCaseString, NestedTypeNode, StructTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EnumStructVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<usize>,

    // Children.
    pub r#struct: NestedTypeNode<StructTypeNode>,
}

impl From<EnumStructVariantTypeNode> for crate::Node {
    fn from(val: EnumStructVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl EnumStructVariantTypeNode {
    pub fn new<T, U>(name: T, r#struct: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<NestedTypeNode<StructTypeNode>>,
    {
        Self {
            name: name.into(),
            discriminator: None,
            r#struct: r#struct.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, StringTypeNode,
        StructFieldTypeNode, U32,
    };

    use super::*;

    #[test]
    fn new() {
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        let node = EnumStructVariantTypeNode::new("my_variant", r#struct);
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, None);
        assert_eq!(
            node.r#struct,
            NestedTypeNode::Value(StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ]))
        );
    }

    #[test]
    fn direct_instantiation() {
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        let node = EnumStructVariantTypeNode {
            name: "my_variant".into(),
            discriminator: Some(42),
            r#struct: NestedTypeNode::Value(r#struct),
        };
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, Some(42));
        assert_eq!(
            node.r#struct,
            NestedTypeNode::Value(StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ]))
        );
    }

    #[test]
    fn new_with_nested_struct() {
        let r#struct = StructTypeNode::new(vec![]);
        let nested_struct =
            PostOffsetTypeNode::pre_offset(PreOffsetTypeNode::absolute(r#struct, 0), 0);
        let node = EnumStructVariantTypeNode::new("my_variant", nested_struct);
        assert_eq!(
            node.r#struct,
            NestedTypeNode::PostOffset(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(PreOffsetTypeNode::absolute(
                    StructTypeNode::new(vec![]),
                    0
                )),
                0,
            ))
        );
        assert_eq!(
            node.r#struct.get_nested_type_node(),
            &StructTypeNode::new(vec![])
        );
    }

    #[test]
    fn to_json() {
        let node = EnumStructVariantTypeNode::new("my_variant", StructTypeNode::new(vec![]));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumStructVariantTypeNode","name":"myVariant","struct":{"kind":"structTypeNode","fields":[]}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumStructVariantTypeNode","name":"myVariant","struct":{"kind":"structTypeNode","fields":[]}}"#;
        let node: EnumStructVariantTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            EnumStructVariantTypeNode::new("my_variant", StructTypeNode::new(vec![]))
        );
    }
}
