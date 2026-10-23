use crate::{CamelCaseString, Docs, StructFieldTypeNode, TypeNode};

impl StructFieldTypeNode {
    pub fn new<T, U>(name: T, r#type: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            identifier: name.into(),
            default_value_strategy: None,
            docs: Docs::default(),
            r#type: Box::new(r#type.into()),
            default_value: Box::new(None),
            display: None,
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DefaultValueStrategy, StringTypeNode, StringValueNode};

    fn hello_value() -> StringValueNode {
        StringValueNode {
            string: "Hello".to_string(),
            plugins: vec![],
        }
    }

    #[test]
    fn new() {
        let node = StructFieldTypeNode::new("my_field", StringTypeNode::utf8());
        assert_eq!(node.identifier, CamelCaseString::new("myField"));
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
    }

    #[test]
    fn direct_instantiation() {
        let node = StructFieldTypeNode {
            identifier: "myField".into(),
            default_value_strategy: Some(DefaultValueStrategy::Optional),
            docs: vec!["Hello".to_string()].into(),
            r#type: Box::new(StringTypeNode::utf8().into()),
            default_value: Box::new(Some(hello_value().into())),
            display: None,
            plugins: vec![],
        };

        assert_eq!(node.identifier, CamelCaseString::new("myField"));
        assert_eq!(
            node.default_value_strategy,
            Some(DefaultValueStrategy::Optional)
        );
        assert_eq!(node.docs, vec!["Hello".to_string()].into());
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(*node.default_value, Some(hello_value().into()));
    }

    #[test]
    fn to_json() {
        let node = StructFieldTypeNode::new("myField", StringTypeNode::utf8());
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldTypeNode","identifier":"myField","type":{"kind":"stringTypeNode","encoding":"utf8"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structFieldTypeNode","identifier":"myField","type":{"kind":"stringTypeNode","encoding":"utf8"}}"#;
        let node: StructFieldTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructFieldTypeNode::new("myField", StringTypeNode::utf8())
        );
    }

    #[test]
    fn to_json_full() {
        let node = StructFieldTypeNode {
            identifier: "myField".into(),
            default_value_strategy: Some(DefaultValueStrategy::Optional),
            docs: vec!["Hello".to_string()].into(),
            r#type: Box::new(StringTypeNode::utf8().into()),
            default_value: Box::new(Some(hello_value().into())),
            display: None,
            plugins: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldTypeNode","identifier":"myField","defaultValueStrategy":"optional","docs":"Hello","type":{"kind":"stringTypeNode","encoding":"utf8"},"defaultValue":{"kind":"stringValueNode","string":"Hello"}}"#
        );
    }

    #[test]
    fn from_json_full() {
        let json = r#"{"kind":"structFieldTypeNode","identifier":"myField","defaultValueStrategy":"optional","docs":"Hello","type":{"kind":"stringTypeNode","encoding":"utf8"},"defaultValue":{"kind":"stringValueNode","string":"Hello"}}"#;
        let node: StructFieldTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructFieldTypeNode {
                identifier: "myField".into(),
                default_value_strategy: Some(DefaultValueStrategy::Optional),
                docs: vec!["Hello".to_string()].into(),
                r#type: Box::new(StringTypeNode::utf8().into()),
                default_value: Box::new(Some(hello_value().into())),
                display: None,
                plugins: vec![],
            }
        );
    }
}
