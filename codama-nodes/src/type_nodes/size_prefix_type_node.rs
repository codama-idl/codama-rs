use crate::{
    NestedTypeNode, NestedTypeNodeTrait, NumberTypeNode, TypeNode, TypeNodeTrait,
    TypeNodeUnionTrait,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes_derive::nestable_type_node;

#[nestable_type_node]
pub struct SizePrefixTypeNode<T: TypeNodeUnionTrait> {
    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: Box<T>,
    pub prefix: Box<NestedTypeNode<NumberTypeNode>>,
}

impl From<SizePrefixTypeNode<crate::TypeNode>> for crate::Node {
    fn from(val: SizePrefixTypeNode<crate::TypeNode>) -> Self {
        crate::Node::Type(val.into())
    }
}

impl<T: TypeNodeTrait> From<SizePrefixTypeNode<NestedTypeNode<T>>>
    for SizePrefixTypeNode<TypeNode>
{
    fn from(node: SizePrefixTypeNode<NestedTypeNode<T>>) -> Self {
        SizePrefixTypeNode {
            r#type: Box::new(TypeNode::from(*node.r#type)),
            prefix: node.prefix,
        }
    }
}

impl<T: TypeNodeTrait> TryFrom<SizePrefixTypeNode<TypeNode>>
    for SizePrefixTypeNode<NestedTypeNode<T>>
{
    type Error = CodamaError;
    fn try_from(node: SizePrefixTypeNode<TypeNode>) -> CodamaResult<Self> {
        Ok(SizePrefixTypeNode {
            r#type: Box::new(NestedTypeNode::try_from(*node.r#type)?),
            prefix: node.prefix,
        })
    }
}

impl<T: TypeNodeUnionTrait> SizePrefixTypeNode<T> {
    pub fn new<U, V>(r#type: U, prefix: V) -> Self
    where
        U: Into<T>,
        V: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self {
            r#type: Box::new(r#type.into()),
            prefix: Box::new(prefix.into()),
        }
    }
}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for SizePrefixTypeNode<NestedTypeNode<T>> {
    type Mapped<U: TypeNodeTrait> = SizePrefixTypeNode<NestedTypeNode<U>>;

    fn get_nested_type_node(&self) -> &T {
        self.r#type.get_nested_type_node()
    }

    fn try_map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> CodamaResult<U>>(
        self,
        f: F,
    ) -> CodamaResult<Self::Mapped<U>> {
        Ok(SizePrefixTypeNode {
            r#type: Box::new(self.r#type.try_map_nested_type_node(f)?),
            prefix: self.prefix,
        })
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
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(*node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U32)));
    }

    #[test]
    fn new_nested_type_node() {
        let node = SizePrefixTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::utf8(),
            NumberTypeNode::le(U32),
        );
        assert_eq!(*node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(*node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U32)));
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
