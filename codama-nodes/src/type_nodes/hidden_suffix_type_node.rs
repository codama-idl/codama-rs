use crate::{NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct HiddenSuffixTypeNode<T: TypeNodeEnumTrait> {
    // Children.
    pub r#type: T,
    pub suffix: Vec<()>, // TODO: ConstantValueNode
}

impl<T: TypeNodeEnumTrait> HiddenSuffixTypeNode<T> {
    pub fn new<U>(r#type: U, suffix: Vec<()>) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: r#type.into(),
            suffix,
        }
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for HiddenSuffixTypeNode<T>
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
        let node = HiddenSuffixTypeNode::<TypeNode>::new(StringTypeNode::utf8(), vec![]);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.suffix, vec![]);
    }

    #[test]
    fn new_nested_type_node() {
        let node = HiddenSuffixTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::utf8(),
            vec![],
        );
        assert_eq!(node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(node.suffix, vec![]);
    }
}
