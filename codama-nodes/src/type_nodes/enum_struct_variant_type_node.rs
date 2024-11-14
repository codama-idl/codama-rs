use crate::CamelCaseString;
use codama_nodes_derive::Node;

use super::{NestedTypeNode, StructTypeNode};

#[derive(Node, Debug, PartialEq)]
pub struct EnumStructVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    pub discriminator: Option<usize>,

    // Children.
    pub r#struct: NestedTypeNode<StructTypeNode>,
}

impl EnumStructVariantTypeNode {
    pub fn new<T, U>(name: T, r#struct: U, discriminator: Option<usize>) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<NestedTypeNode<StructTypeNode>>,
    {
        Self {
            name: name.into(),
            discriminator,
            r#struct: r#struct.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, StringTypeNode,
        StructFieldTypeNode, U32,
    };

    use super::*;

    #[test]
    fn new() {
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        let node = EnumStructVariantTypeNode::new("my_variant", r#struct, Some(42));
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, Some(42));
        assert_eq!(
            node.r#struct,
            NestedTypeNode::Value(StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ]))
        );
    }

    #[test]
    fn new_with_nested_struct() {
        let r#struct = StructTypeNode::new(vec![]);
        let nested_struct =
            PostOffsetTypeNode::pre_offset(PreOffsetTypeNode::absolute(r#struct, 0), 0);
        let node = EnumStructVariantTypeNode::new("my_variant", nested_struct, None);
        assert_eq!(
            node.r#struct,
            NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                    StructTypeNode::new(vec![]),
                    0
                ))),
                0,
            )))
        );
        assert_eq!(
            node.r#struct.get_nested_type_node(),
            &StructTypeNode::new(vec![])
        );
    }
}
