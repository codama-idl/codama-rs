use crate::{CamelCaseString, DiscriminatorNode, Docs, HasName, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EventNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub data: TypeNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub discriminators: Vec<DiscriminatorNode>,
}

impl EventNode {
    pub fn new<T, U>(name: T, data: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            name: name.into(),
            docs: Docs::default(),
            data: data.into(),
            discriminators: vec![],
        }
    }
}

impl HasName for EventNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, U8};

    #[test]
    fn new() {
        let node = EventNode::new(
            "myEvent",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]),
        );

        assert_eq!(node.name, CamelCaseString::new("myEvent"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(
            node.data,
            TypeNode::Struct(StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]))
        );
        assert_eq!(node.discriminators, vec![]);
    }

    #[test]
    fn direct_instantiation() {
        let node = EventNode {
            name: "myEvent".into(),
            docs: Docs::default(),
            data: StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ])
            .into(),
            discriminators: vec![],
        };

        assert_eq!(node.name, CamelCaseString::new("myEvent"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(
            node.data,
            TypeNode::Struct(StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]))
        );
        assert_eq!(node.discriminators, vec![]);
    }

    #[test]
    fn to_json() {
        let node = EventNode::new(
            "myEvent",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            ]),
        );

        let json = serde_json::to_string(&node).unwrap();

        assert_eq!(
            json,
            r#"{"kind":"eventNode","name":"myEvent","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}},{"kind":"structFieldTypeNode","name":"age","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}}]}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"eventNode","name":"myEvent","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}},{"kind":"structFieldTypeNode","name":"age","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}}]}}"#;

        let node: EventNode = serde_json::from_str(json).unwrap();

        assert_eq!(
            node,
            EventNode::new(
                "myEvent",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                    StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
                ]),
            )
        );
    }
}
