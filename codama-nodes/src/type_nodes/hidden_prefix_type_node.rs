use crate::{ConstantValueNode, NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::type_node;

#[type_node]
pub struct HiddenPrefixTypeNode<T: TypeNodeEnumTrait> {
    // Children.
    pub r#type: T,
    pub prefix: Vec<ConstantValueNode>,
}

impl<T: TypeNodeEnumTrait> HiddenPrefixTypeNode<T> {
    pub fn new<U>(r#type: U, prefix: Vec<ConstantValueNode>) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: r#type.into(),
            prefix,
        }
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for HiddenPrefixTypeNode<T>
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
        let node = HiddenPrefixTypeNode::<TypeNode>::new(
            StringTypeNode::utf8(),
            vec![ConstantValueNode::bytes(Base16, "ffff")],
        );
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.prefix, vec![ConstantValueNode::bytes(Base16, "ffff")]);
    }

    #[test]
    fn new_nested_type_node() {
        let node = HiddenPrefixTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::utf8(),
            vec![],
        );
        assert_eq!(node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(node.prefix, vec![]);
    }
}
