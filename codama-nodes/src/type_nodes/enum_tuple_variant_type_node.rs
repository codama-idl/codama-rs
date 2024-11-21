use crate::{CamelCaseString, NestedTypeNode, TupleTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EnumTupleVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<usize>,

    // Children.
    pub tuple: NestedTypeNode<TupleTypeNode>,
}

impl EnumTupleVariantTypeNode {
    pub fn new<T, U>(name: T, tuple: U, discriminator: Option<usize>) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<NestedTypeNode<TupleTypeNode>>,
    {
        Self {
            name: name.into(),
            discriminator,
            tuple: tuple.into(),
        }
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
        let node = EnumTupleVariantTypeNode::new("my_variant", tuple, Some(42));
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
        let node = EnumTupleVariantTypeNode::new("my_variant", nested_struct, None);
        assert_eq!(
            node.tuple,
            NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                    TupleTypeNode::new(vec![]),
                    0
                ))),
                0,
            )))
        );
        assert_eq!(
            node.tuple.get_nested_type_node(),
            &TupleTypeNode::new(vec![])
        );
    }

    #[test]
    fn to_json() {
        let node = EnumTupleVariantTypeNode::new("my_variant", TupleTypeNode::new(vec![]), None);
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
            EnumTupleVariantTypeNode::new("my_variant", TupleTypeNode::new(vec![]), None)
        );
    }
}
