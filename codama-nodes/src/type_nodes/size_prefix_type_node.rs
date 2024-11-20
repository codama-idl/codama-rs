use crate::{
    NestedTypeNode, NestedTypeNodeTrait, NumberTypeNode, TypeNodeEnumTrait, TypeNodeTrait,
};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq, Clone)]
pub struct SizePrefixTypeNode<T: TypeNodeEnumTrait> {
    // Children.
    pub r#type: T,
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl<T: TypeNodeEnumTrait> SizePrefixTypeNode<T> {
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

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for SizePrefixTypeNode<T>
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
}
