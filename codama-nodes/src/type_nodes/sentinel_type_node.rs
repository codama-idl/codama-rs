use crate::{
    ConstantValueNode, NestedTypeNode, NestedTypeNodeTrait, TypeNodeTrait, TypeNodeUnionTrait,
};
use codama_nodes_derive::nestable_type_node;

#[nestable_type_node]
pub struct SentinelTypeNode<T: TypeNodeUnionTrait> {
    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: T,
    pub sentinel: ConstantValueNode,
}

impl From<SentinelTypeNode<crate::TypeNode>> for crate::Node {
    fn from(val: SentinelTypeNode<crate::TypeNode>) -> Self {
        crate::Node::Type(val.into())
    }
}

impl<T: TypeNodeUnionTrait> SentinelTypeNode<T> {
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

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for SentinelTypeNode<NestedTypeNode<T>> {
    type Mapped<U: TypeNodeTrait> = SentinelTypeNode<NestedTypeNode<U>>;

    fn get_nested_type_node(&self) -> &T {
        self.r#type.get_nested_type_node()
    }

    fn map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> U>(self, f: F) -> Self::Mapped<U> {
        SentinelTypeNode {
            r#type: self.r#type.map_nested_type_node(f),
            sentinel: self.sentinel,
        }
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

    #[test]
    fn to_json() {
        let node = SentinelTypeNode::<TypeNode>::new(
            StringTypeNode::utf8(),
            ConstantValueNode::bytes(Base16, "ffff"),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"sentinelTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"sentinel":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"sentinelTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"sentinel":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}}"#;
        let node: SentinelTypeNode<TypeNode> = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            SentinelTypeNode::<TypeNode>::new(
                StringTypeNode::utf8(),
                ConstantValueNode::bytes(Base16, "ffff"),
            )
        );
    }
}
