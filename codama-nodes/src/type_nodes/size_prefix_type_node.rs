use crate::{
    NestedTypeNode, NestedTypeNodeTrait, NumberTypeNode, TypeNodeTrait, TypeNodeUnionTrait,
};
use codama_nodes_derive::nestable_type_node;

#[nestable_type_node]
pub struct SizePrefixTypeNode<T: TypeNodeUnionTrait> {
    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: T,
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl From<SizePrefixTypeNode<crate::TypeNode>> for crate::Node {
    fn from(val: SizePrefixTypeNode<crate::TypeNode>) -> Self {
        crate::Node::Type(val.into())
    }
}

impl<T: TypeNodeUnionTrait> SizePrefixTypeNode<T> {
    pub fn new<U, V>(r#type: U, prefix: V) -> Self
    where
        U: Into<T>,
        V: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self {
            r#type: r#type.into(),
            prefix: prefix.into(),
        }
    }
}

impl<T: TypeNodeUnionTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for SizePrefixTypeNode<T>
where
    T: NestedTypeNodeTrait<U>,
{
    fn get_nested_type_node(&self) -> &U {
        self.r#type.get_nested_type_node()
    }
}

#[cfg(test)]
mod tests {
    use crate::{NestedTypeNode, StringTypeNode, TypeNode, U32};

    use super::*;

    #[test]
    fn new_type_node() {
        let node =
            SizePrefixTypeNode::<TypeNode>::new(StringTypeNode::utf8(), NumberTypeNode::le(U32));
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U32)));
    }

    #[test]
    fn new_nested_type_node() {
        let node = SizePrefixTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::utf8(),
            NumberTypeNode::le(U32),
        );
        assert_eq!(node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U32)));
    }

    #[test]
    fn to_json() {
        let node =
            SizePrefixTypeNode::<TypeNode>::new(StringTypeNode::utf8(), NumberTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#;
        let node: SizePrefixTypeNode<TypeNode> = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            SizePrefixTypeNode::<TypeNode>::new(StringTypeNode::utf8(), NumberTypeNode::le(U32))
        );
    }
}
