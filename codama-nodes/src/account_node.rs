use crate::{
    CamelCaseString, DiscriminatorNode, Docs, NestedTypeNode, PdaLinkNode, StructTypeNode,
};
use codama_nodes_derive::node;

#[node]
pub struct AccountNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub size: Option<usize>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub data: NestedTypeNode<StructTypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub pda: Option<PdaLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
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

    #[test]
    fn to_json() {
        let node = AccountNode::new(
            "myAccount",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountNode","name":"myAccount","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}},{"kind":"structFieldTypeNode","name":"age","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}}]}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountNode","name":"myAccount","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}},{"kind":"structFieldTypeNode","name":"age","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}}]}}"#;
        let node: AccountNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            AccountNode::new(
                "myAccount",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                    StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
                ]),
            )
        );
    }
}
