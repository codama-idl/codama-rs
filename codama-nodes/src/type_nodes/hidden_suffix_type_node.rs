use crate::{ConstantValueNode, NestedTypeNodeTrait, TypeNodeUnionTrait, TypeNodeTrait};
use codama_nodes_derive::type_node;

#[type_node]
pub struct HiddenSuffixTypeNode<T: TypeNodeUnionTrait> {
    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: T,
    pub suffix: Vec<ConstantValueNode>,
}

impl Into<crate::Node> for HiddenSuffixTypeNode<crate::TypeNode> {
    fn into(self) -> crate::Node {
        crate::Node::Type(self.into())
    }
}

impl<T: TypeNodeUnionTrait> HiddenSuffixTypeNode<T> {
    pub fn new<U>(r#type: U, suffix: Vec<ConstantValueNode>) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: r#type.into(),
            suffix,
        }
    }
}

impl<T: TypeNodeUnionTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for HiddenSuffixTypeNode<T>
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
        let node = HiddenSuffixTypeNode::<TypeNode>::new(
            StringTypeNode::utf8(),
            vec![ConstantValueNode::bytes(Base16, "ffff")],
        );
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.suffix, vec![ConstantValueNode::bytes(Base16, "ffff")]);
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

    #[test]
    fn to_json() {
        let node = HiddenSuffixTypeNode::<TypeNode>::new(
            StringTypeNode::utf8(),
            vec![ConstantValueNode::bytes(Base16, "ffff")],
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"hiddenSuffixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"suffix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"hiddenSuffixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"suffix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#;
        let node: HiddenSuffixTypeNode<TypeNode> = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            HiddenSuffixTypeNode::<TypeNode>::new(
                StringTypeNode::utf8(),
                vec![ConstantValueNode::bytes(Base16, "ffff")],
            )
        );
    }
}
