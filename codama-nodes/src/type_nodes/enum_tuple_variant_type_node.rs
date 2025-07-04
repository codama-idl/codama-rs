use crate::{CamelCaseString, HasName, NestedTypeNode, TupleTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EnumTupleVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<usize>,

    // Children.
    pub tuple: NestedTypeNode<TupleTypeNode>,
}

impl From<EnumTupleVariantTypeNode> for crate::Node {
    fn from(val: EnumTupleVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl EnumTupleVariantTypeNode {
    pub fn new<T, U>(name: T, tuple: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<NestedTypeNode<TupleTypeNode>>,
    {
        Self {
            name: name.into(),
            discriminator: None,
            tuple: tuple.into(),
        }
    }
}

impl HasName for EnumTupleVariantTypeNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, StringTypeNode,
        U32,
    };

    use super::*;

    #[test]
    fn new() {
        let tuple = TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(),
            StringTypeNode::utf8().into(),
        ]);
        let node = EnumTupleVariantTypeNode::new("my_variant", tuple);
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, None);
        assert_eq!(
            node.tuple,
            NestedTypeNode::Value(TupleTypeNode::new(vec![
                NumberTypeNode::le(U32).into(),
                StringTypeNode::utf8().into(),
            ]))
        );
    }

    #[test]
    fn direct_instantiation() {
        let tuple = TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(),
            StringTypeNode::utf8().into(),
        ]);
        let node = EnumTupleVariantTypeNode {
            name: "my_variant".into(),
            discriminator: Some(42),
            tuple: tuple.into(),
        };
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, Some(42));
        assert_eq!(
            node.tuple,
            NestedTypeNode::Value(TupleTypeNode::new(vec![
                NumberTypeNode::le(U32).into(),
                StringTypeNode::utf8().into(),
            ]))
        );
    }

    #[test]
    fn new_with_nested_struct() {
        let tuple = TupleTypeNode::new(vec![]);
        let nested_struct =
            PostOffsetTypeNode::pre_offset(PreOffsetTypeNode::absolute(tuple, 0), 0);
        let node = EnumTupleVariantTypeNode::new("my_variant", nested_struct);
        assert_eq!(
            node.tuple,
            NestedTypeNode::PostOffset(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(PreOffsetTypeNode::absolute(
                    TupleTypeNode::new(vec![]),
                    0
                )),
                0,
            ))
        );
        assert_eq!(
            node.tuple.get_nested_type_node(),
            &TupleTypeNode::new(vec![])
        );
    }

    #[test]
    fn to_json() {
        let node = EnumTupleVariantTypeNode::new("my_variant", TupleTypeNode::new(vec![]));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumTupleVariantTypeNode","name":"myVariant","tuple":{"kind":"tupleTypeNode","items":[]}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumTupleVariantTypeNode","name":"myVariant","tuple":{"kind":"tupleTypeNode","items":[]}}"#;
        let node: EnumTupleVariantTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            EnumTupleVariantTypeNode::new("my_variant", TupleTypeNode::new(vec![]))
        );
    }
}
