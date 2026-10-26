use crate::{CamelCaseString, ConstantNode, Docs, TypeNode, ValueNode};

impl ConstantNode {
    pub fn new<T, U, V>(identifier: T, r#type: U, value: V) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
        V: Into<ValueNode>,
    {
        Self {
            identifier: identifier.into(),
            docs: Docs::default(),
            r#type: Box::new(r#type.into()),
            value: Box::new(value.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerTypeNode, IntegerValueNode, StringTypeNode, StringValueNode, U64};
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        let node = ConstantNode::new(
            "max_items",
            IntegerTypeNode::le(U64),
            IntegerValueNode::new(100u64),
        );
        assert_eq!(node.identifier, CamelCaseString::new("maxItems"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(*node.r#type, TypeNode::Integer(IntegerTypeNode::le(U64)));
        assert_eq!(
            *node.value,
            ValueNode::Integer(IntegerValueNode::new(100u64))
        );
    }

    #[test]
    fn direct_instantiation() {
        let node = ConstantNode {
            identifier: "appName".into(),
            docs: Docs::default(),
            r#type: Box::new(StringTypeNode::utf8().into()),
            value: Box::new(StringValueNode::new("MyApp").into()),
            plugins: vec![],
        };
        assert_eq!(node.identifier, CamelCaseString::new("appName"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(
            *node.value,
            ValueNode::String(StringValueNode::new("MyApp"))
        );
    }

    #[test]
    fn to_json() {
        let node = ConstantNode::new(
            "maxItems",
            IntegerTypeNode::le(U64),
            IntegerValueNode::new(100u64),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantNode","identifier":"maxItems","type":{"kind":"integerTypeNode","format":"u64","endian":"le"},"value":{"kind":"integerValueNode","value":"100"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"constantNode","identifier":"maxItems","type":{"kind":"integerTypeNode","format":"u64","endian":"le"},"value":{"kind":"integerValueNode","value":"100"}}"#;
        let node: ConstantNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConstantNode::new(
                "maxItems",
                IntegerTypeNode::le(U64),
                IntegerValueNode::new(100u64)
            )
        );
    }

    #[test]
    fn to_json_with_docs() {
        let mut node = ConstantNode::new(
            "maxItems",
            IntegerTypeNode::le(U64),
            IntegerValueNode::new(100u64),
        );
        node.docs = vec!["Maximum number of items.".to_string()].into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantNode","identifier":"maxItems","docs":"Maximum number of items.","type":{"kind":"integerTypeNode","format":"u64","endian":"le"},"value":{"kind":"integerValueNode","value":"100"}}"#
        );
    }
}
