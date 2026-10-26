use crate::{AccountNode, CamelCaseString, Docs, TypeNode};

impl AccountNode {
    pub fn new<T, U>(identifier: T, data: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            identifier: identifier.into(),
            size: None,
            docs: Docs::default(),
            data: Box::new(data.into()),
            pda: None,
            discriminators: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DefinedTypeLinkNode, IntegerTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
        U8,
    };

    #[test]
    fn new() {
        let node = AccountNode::new(
            "myAccount",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
            ]),
        );
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
        assert_eq!(node.size, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(
            *node.data,
            TypeNode::Struct(StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
            ]))
        );
        assert_eq!(node.pda, None);
        assert_eq!(node.discriminators, vec![]);
    }

    #[test]
    fn new_with_any_type() {
        let node = AccountNode::new("myAccount", DefinedTypeLinkNode::new("myData"));
        assert_eq!(
            *node.data,
            TypeNode::Link(DefinedTypeLinkNode::new("myData"))
        );
    }

    #[test]
    fn direct_instantiation() {
        let node = AccountNode {
            identifier: "myAccount".into(),
            size: None,
            docs: Docs::default(),
            data: Box::new(
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                    StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
                ])
                .into(),
            ),
            pda: None,
            discriminators: vec![],
            plugins: vec![],
        };
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
        assert_eq!(node.size, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(
            *node.data,
            TypeNode::Struct(StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
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
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
            ]),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountNode","identifier":"myAccount","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","identifier":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}},{"kind":"structFieldTypeNode","identifier":"age","type":{"kind":"integerTypeNode","format":"u8","endian":"le"}}]}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountNode","identifier":"myAccount","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","identifier":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}},{"kind":"structFieldTypeNode","identifier":"age","type":{"kind":"integerTypeNode","format":"u8","endian":"le"}}]}}"#;
        let node: AccountNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            AccountNode::new(
                "myAccount",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                    StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
                ]),
            )
        );
    }

    #[test]
    fn to_json_with_any_type() {
        let node = AccountNode::new("myAccount", DefinedTypeLinkNode::new("myData"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountNode","identifier":"myAccount","data":{"kind":"definedTypeLinkNode","identifier":"myData"}}"#
        );
    }

    #[test]
    fn from_json_with_any_type() {
        let json = r#"{"kind":"accountNode","identifier":"myAccount","data":{"kind":"definedTypeLinkNode","identifier":"myData"}}"#;
        let node: AccountNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            AccountNode::new("myAccount", DefinedTypeLinkNode::new("myData"))
        );
    }
}
