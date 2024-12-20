use crate::{
    ConstantValueNode, NestedTypeNode, NestedTypeNodeTrait, TypeNode, TypeNodeTrait,
    TypeNodeUnionTrait,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes_derive::nestable_type_node;

#[nestable_type_node]
pub struct HiddenSuffixTypeNode<T: TypeNodeUnionTrait> {
    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: Box<T>,
    pub suffix: Vec<ConstantValueNode>,
}

impl From<HiddenSuffixTypeNode<crate::TypeNode>> for crate::Node {
    fn from(val: HiddenSuffixTypeNode<crate::TypeNode>) -> Self {
        crate::Node::Type(val.into())
    }
}

impl<T: TypeNodeTrait> From<HiddenSuffixTypeNode<NestedTypeNode<T>>>
    for HiddenSuffixTypeNode<TypeNode>
{
    fn from(node: HiddenSuffixTypeNode<NestedTypeNode<T>>) -> Self {
        HiddenSuffixTypeNode {
            r#type: Box::new(TypeNode::from(*node.r#type)),
            suffix: node.suffix,
        }
    }
}

impl<T: TypeNodeTrait> TryFrom<HiddenSuffixTypeNode<TypeNode>>
    for HiddenSuffixTypeNode<NestedTypeNode<T>>
{
    type Error = CodamaError;
    fn try_from(node: HiddenSuffixTypeNode<TypeNode>) -> CodamaResult<Self> {
        Ok(HiddenSuffixTypeNode {
            r#type: Box::new(NestedTypeNode::try_from(*node.r#type)?),
            suffix: node.suffix,
        })
    }
}

impl<T: TypeNodeUnionTrait> HiddenSuffixTypeNode<T> {
    pub fn new<U>(r#type: U, suffix: Vec<ConstantValueNode>) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: Box::new(r#type.into()),
            suffix,
        }
    }
}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for HiddenSuffixTypeNode<NestedTypeNode<T>> {
    type Mapped<U: TypeNodeTrait> = HiddenSuffixTypeNode<NestedTypeNode<U>>;

    fn get_nested_type_node(&self) -> &T {
        self.r#type.get_nested_type_node()
    }

    fn map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> U>(self, f: F) -> Self::Mapped<U> {
        HiddenSuffixTypeNode {
            r#type: Box::new(self.r#type.map_nested_type_node(f)),
            suffix: self.suffix,
        }
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
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.suffix, vec![ConstantValueNode::bytes(Base16, "ffff")]);
    }

    #[test]
    fn new_nested_type_node() {
        let node = HiddenSuffixTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::utf8(),
            vec![],
        );
        assert_eq!(*node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
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
