use crate::{
    CamelCaseString, DiscriminatorNode, Docs, NestedTypeNode, PdaLinkNode, StructTypeNode,
};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone)]
pub struct AccountNode {
    // Data.
    pub name: CamelCaseString,
    pub size: Option<usize>,
    pub docs: Docs,

    // Children.
    pub data: NestedTypeNode<StructTypeNode>,
    pub pda: Option<PdaLinkNode>,
    pub discriminators: Vec<DiscriminatorNode>,
}

impl AccountNode {
    pub fn new<T, U>(name: T, data: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<NestedTypeNode<StructTypeNode>>,
    {
        Self {
            name: name.into(),
            size: None,
            docs: Docs::default(),
            data: data.into(),
            pda: None,
            discriminators: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, StructFieldTypeNode, U8};

    #[test]
    fn new() {
        let node = AccountNode::new(
            "myAccount",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]),
        );
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert_eq!(node.size, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(
            node.data,
            NestedTypeNode::Value(StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]))
        );
        assert_eq!(node.pda, None);
        assert_eq!(node.discriminators, vec![]);
    }

    #[test]
    fn direct_instantiation() {
        let node = AccountNode {
            name: "myAccount".into(),
            size: None,
            docs: Docs::default(),
            data: StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ])
            .into(),
            pda: None,
            discriminators: vec![],
        };
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert_eq!(node.size, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(
            node.data,
            NestedTypeNode::Value(StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]))
        );
        assert_eq!(node.pda, None);
        assert_eq!(node.discriminators, vec![]);
    }
}
