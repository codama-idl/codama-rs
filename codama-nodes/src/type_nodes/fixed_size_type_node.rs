use crate::{NestedTypeNode, NestedTypeNodeTrait, TypeNode, TypeNodeTrait, TypeNodeUnionTrait};
use codama_nodes_derive::nestable_type_node;

#[nestable_type_node]
pub struct FixedSizeTypeNode<T: TypeNodeUnionTrait> {
    // Data.
    pub size: usize,

    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: Box<T>,
}

impl From<FixedSizeTypeNode<crate::TypeNode>> for crate::Node {
    fn from(val: FixedSizeTypeNode<crate::TypeNode>) -> Self {
        crate::Node::Type(val.into())
    }
}

impl<T: TypeNodeUnionTrait> FixedSizeTypeNode<T> {
    pub fn new<U>(r#type: U, size: usize) -> Self
    where
        U: Into<T>,
    {
        Self {
            size,
            r#type: Box::new(r#type.into()),
        }
    }
}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for FixedSizeTypeNode<NestedTypeNode<T>> {
    type Mapped<U: TypeNodeTrait> = FixedSizeTypeNode<NestedTypeNode<U>>;

    fn get_nested_type_node(&self) -> &T {
        self.r#type.get_nested_type_node()
    }

    fn map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> U>(self, f: F) -> Self::Mapped<U> {
        FixedSizeTypeNode {
            size: self.size,
            r#type: Box::new(self.r#type.map_nested_type_node(f)),
        }
    }
}

impl<T: TypeNodeTrait> TypeNodeTrait for FixedSizeTypeNode<NestedTypeNode<T>> {
    fn into_type_node(self) -> TypeNode {
        TypeNode::FixedSize(FixedSizeTypeNode {
            size: self.size,
            r#type: Box::new((*self.r#type).into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BooleanTypeNode, NestedTypeNode, NumberFormat::*, NumberTypeNode, StringTypeNode, TypeNode,
    };

    use super::*;

    #[test]
    fn new_type_node() {
        let node = FixedSizeTypeNode::<TypeNode>::new(StringTypeNode::utf8(), 42);
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.size, 42);
    }

    #[test]
    fn new_nested_type_node() {
        let node =
            FixedSizeTypeNode::<NestedTypeNode<StringTypeNode>>::new(StringTypeNode::utf8(), 42);
        assert_eq!(*node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(node.size, 42);
    }

    #[test]
    fn map_nested_type_node() {
        let node = FixedSizeTypeNode::<NestedTypeNode<_>>::new(NumberTypeNode::le(U32), 42);
        let node = node.map_nested_type_node(|node| BooleanTypeNode::new(node));
        assert_eq!(
            node,
            FixedSizeTypeNode::<NestedTypeNode<_>>::new(
                BooleanTypeNode::new(NumberTypeNode::le(U32)),
                42
            )
        );
    }

    #[test]
    fn to_json_type_node() {
        let node = FixedSizeTypeNode::<TypeNode>::new(StringTypeNode::utf8(), 42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"fixedSizeTypeNode","size":42,"type":{"kind":"stringTypeNode","encoding":"utf8"}}"#
        );
    }

    #[test]
    fn from_json_type_node() {
        let json = r#"{"kind":"fixedSizeTypeNode","size":42,"type":{"kind":"stringTypeNode","encoding":"utf8"}}"#;
        let node: FixedSizeTypeNode<TypeNode> = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            FixedSizeTypeNode::<TypeNode>::new(StringTypeNode::utf8(), 42)
        );
    }

    #[test]
    fn to_json_nested_type_node() {
        let node =
            FixedSizeTypeNode::<NestedTypeNode<StringTypeNode>>::new(StringTypeNode::utf8(), 42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"fixedSizeTypeNode","size":42,"type":{"kind":"stringTypeNode","encoding":"utf8"}}"#
        );
    }

    #[test]
    fn from_json_nested_type_node() {
        let json = r#"{"kind":"fixedSizeTypeNode","size":42,"type":{"kind":"stringTypeNode","encoding":"utf8"}}"#;
        let node: FixedSizeTypeNode<NestedTypeNode<StringTypeNode>> =
            serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            FixedSizeTypeNode::<NestedTypeNode<StringTypeNode>>::new(StringTypeNode::utf8(), 42)
        );
    }
}
