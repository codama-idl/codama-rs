use crate::{CamelCaseString, Docs, StructFieldTypeNode, TypeNode};

impl StructFieldTypeNode {
    pub fn new<T, U>(name: T, r#type: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            name: name.into(),
            default_value_strategy: None,
            docs: Docs::default(),
            r#type: Box::new(r#type.into()),
            default_value: Box::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DefaultValueStrategy, NumberTypeNode, NumberValueNode, U32};

    #[test]
    fn new() {
        let node = StructFieldTypeNode::new("my_field", NumberTypeNode::le(U32));
        assert_eq!(node.name, CamelCaseString::new("myField"));
        assert_eq!(*node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
    }

    #[test]
    fn direct_instantiation() {
        let node = StructFieldTypeNode {
            name: "myField".into(),
            default_value_strategy: Some(DefaultValueStrategy::Optional),
            docs: vec!["Hello".to_string()].into(),
            r#type: Box::new(NumberTypeNode::le(U32).into()),
            default_value: Box::new(Some(NumberValueNode::new(42u32).into())),
        };

        assert_eq!(node.name, CamelCaseString::new("myField"));
        assert_eq!(
            node.default_value_strategy,
            Some(DefaultValueStrategy::Optional)
        );
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(*node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
        assert_eq!(
            *node.default_value,
            Some(NumberValueNode::new(42u32).into())
        );
    }

    #[test]
    fn to_json() {
        let node = StructFieldTypeNode::new("myField", NumberTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldTypeNode","name":"myField","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structFieldTypeNode","name":"myField","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#;
        let node: StructFieldTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructFieldTypeNode::new("myField", NumberTypeNode::le(U32))
        );
    }

    #[test]
    fn to_json_full() {
        let node = StructFieldTypeNode {
            name: "myField".into(),
            default_value_strategy: Some(DefaultValueStrategy::Optional),
            docs: vec!["Hello".to_string()].into(),
            r#type: Box::new(NumberTypeNode::le(U32).into()),
            default_value: Box::new(Some(NumberValueNode::new(42u32).into())),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldTypeNode","name":"myField","defaultValueStrategy":"optional","docs":["Hello"],"type":{"kind":"numberTypeNode","format":"u32","endian":"le"},"defaultValue":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json_full() {
        let json = r#"{"kind":"structFieldTypeNode","name":"myField","defaultValueStrategy":"optional","docs":["Hello"],"type":{"kind":"numberTypeNode","format":"u32","endian":"le"},"defaultValue":{"kind":"numberValueNode","number":42}}"#;
        let node: StructFieldTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructFieldTypeNode {
                name: "myField".into(),
                default_value_strategy: Some(DefaultValueStrategy::Optional),
                docs: vec!["Hello".to_string()].into(),
                r#type: Box::new(NumberTypeNode::le(U32).into()),
                default_value: Box::new(Some(NumberValueNode::new(42u32).into())),
            }
        );
    }
}
