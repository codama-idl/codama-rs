use crate::{NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::{node, TypeNode};

#[node]
#[derive(TypeNode)]
pub struct FixedSizeTypeNode<T: TypeNodeEnumTrait> {
    // Data.
    pub size: usize,

    // Children.
    pub r#type: T,
}

impl<T: TypeNodeEnumTrait> FixedSizeTypeNode<T> {
    pub fn new<U>(r#type: U, size: usize) -> Self
    where
        U: Into<T>,
    {
        Self {
            size,
            r#type: r#type.into(),
        }
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for FixedSizeTypeNode<T>
where
    T: NestedTypeNodeTrait<U>,
{
    fn get_nested_type_node(&self) -> &U {
        self.r#type.get_nested_type_node()
    }
}

#[cfg(test)]
mod tests {
    use crate::{NestedTypeNode, StringTypeNode, TypeNode};

    use super::*;

    #[test]
    fn new_type_node() {
        let node = FixedSizeTypeNode::<TypeNode>::new(StringTypeNode::utf8(), 42);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.size, 42);
    }

    #[test]
    fn new_nested_type_node() {
        let node =
            FixedSizeTypeNode::<NestedTypeNode<StringTypeNode>>::new(StringTypeNode::utf8(), 42);
        assert_eq!(node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(node.size, 42);
    }
}
