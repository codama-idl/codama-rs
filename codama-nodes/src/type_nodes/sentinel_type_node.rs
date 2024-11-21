use crate::{ConstantValueNode, NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::type_node;

#[type_node]
pub struct SentinelTypeNode<T: TypeNodeEnumTrait> {
    // Children.
    pub r#type: T,
    pub sentinel: ConstantValueNode,
}

impl<T: TypeNodeEnumTrait> SentinelTypeNode<T> {
    pub fn new<U>(r#type: U, sentinel: ConstantValueNode) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: r#type.into(),
            sentinel,
        }
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for SentinelTypeNode<T>
where
    T: NestedTypeNodeTrait<U>,
{
    fn get_nested_type_node(&self) -> &U {
        self.r#type.get_nested_type_node()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base16, NestedTypeNode, StringTypeNode, TypeNode};

    #[test]
    fn new_type_node() {
        let node = SentinelTypeNode::<TypeNode>::new(
            StringTypeNode::utf8(),
            ConstantValueNode::bytes(Base16, "ffff"),
        );
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.sentinel, ConstantValueNode::bytes(Base16, "ffff"));
    }

    #[test]
    fn new_nested_type_node() {
        let node = SentinelTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::utf8(),
            ConstantValueNode::bytes(Base16, "ffff"),
        );
        assert_eq!(node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(node.sentinel, ConstantValueNode::bytes(Base16, "ffff"));
    }
}
