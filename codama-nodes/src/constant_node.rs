use crate::{CamelCaseString, Docs, HasName, TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantNode {
    // data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // children.
    pub r#type: TypeNode,
    pub value: ValueNode,
}

impl ConstantNode {
    pub fn new<T, U, V>(name: T, r#type: U, value: V) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
        V: Into<ValueNode>,
    {
        Self {
            name: name.into(),
            docs: Docs::default(),
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}

impl HasName for ConstantNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, NumberValueNode, StringTypeNode, StringValueNode, U64};
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        let node = ConstantNode::new(
            "max_items",
            NumberTypeNode::le(U64),
            NumberValueNode::new(100u64),
        );
        assert_eq!(node.name, CamelCaseString::new("maxItems"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(100u64)));
    }

    #[test]
    fn direct_instantiation() {
        let node = ConstantNode {
            name: "appName".into(),
            docs: Docs::default(),
            r#type: StringTypeNode::utf8().into(),
            value: StringValueNode::new("MyApp").into(),
        };
        assert_eq!(node.name, CamelCaseString::new("appName"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.value, ValueNode::String(StringValueNode::new("MyApp")));
    }

    #[test]
    fn to_json() {
        let node = ConstantNode::new(
            "maxItems",
            NumberTypeNode::le(U64),
            NumberValueNode::new(100u64),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantNode","name":"maxItems","type":{"kind":"numberTypeNode","format":"u64","endian":"le"},"value":{"kind":"numberValueNode","number":100}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"constantNode","name":"maxItems","type":{"kind":"numberTypeNode","format":"u64","endian":"le"},"value":{"kind":"numberValueNode","number":100}}"#;
        let node: ConstantNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConstantNode::new(
                "maxItems",
                NumberTypeNode::le(U64),
                NumberValueNode::new(100u64)
            )
        );
    }

    #[test]
    fn to_json_with_docs() {
        let mut node = ConstantNode::new(
            "maxItems",
            NumberTypeNode::le(U64),
            NumberValueNode::new(100u64),
        );
        node.docs = Docs::new().add_doc("Maximum number of items.");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantNode","name":"maxItems","docs":["Maximum number of items."],"type":{"kind":"numberTypeNode","format":"u64","endian":"le"},"value":{"kind":"numberValueNode","number":100}}"#
        );
    }
}
