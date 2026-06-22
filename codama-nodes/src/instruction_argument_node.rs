use crate::{
    CamelCaseString, Docs, InstructionArgumentNode, InstructionInputValueNode, StructFieldTypeNode,
    StructTypeNode, TypeNode,
};

impl InstructionArgumentNode {
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
            display: None,
        }
    }
}

impl From<StructFieldTypeNode> for InstructionArgumentNode {
    fn from(value: StructFieldTypeNode) -> Self {
        Self {
            name: value.name,
            default_value_strategy: value.default_value_strategy,
            docs: value.docs,
            // `StructFieldTypeNode.r#type` is `Box<TypeNode>`;
            // `InstructionArgumentNode.r#type` is also `Box<TypeNode>`,
            // so forward the existing box.
            r#type: value.r#type,
            default_value: Box::new((*value.default_value).map(InstructionInputValueNode::from)),
            // Both nodes carry display metadata as `structFieldDisplayNode`,
            // so the member's presentation survives the conversion.
            display: value.display,
        }
    }
}

impl From<StructTypeNode> for Vec<InstructionArgumentNode> {
    fn from(val: StructTypeNode) -> Self {
        val.fields
            .into_iter()
            .map(InstructionArgumentNode::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArgumentValueNode, DefaultValueStrategy, NumberTypeNode, U32};

    #[test]
    fn new() {
        let node = InstructionArgumentNode::new("my_argument", NumberTypeNode::le(U32));
        assert_eq!(node.name, CamelCaseString::new("myArgument"));
        assert_eq!(*node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
    }

    #[test]
    fn direct_instantiation() {
        let node = InstructionArgumentNode {
            name: "myArgument".into(),
            default_value_strategy: Some(DefaultValueStrategy::Optional),
            docs: vec!["Hello".to_string()].into(),
            r#type: Box::new(NumberTypeNode::le(U32).into()),
            default_value: Box::new(Some(ArgumentValueNode::new("myOtherArgument").into())),
            display: None,
        };

        assert_eq!(node.name, CamelCaseString::new("myArgument"));
        assert_eq!(
            node.default_value_strategy,
            Some(DefaultValueStrategy::Optional)
        );
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(*node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
        assert_eq!(
            *node.default_value,
            Some(InstructionInputValueNode::ArgumentValue(
                ArgumentValueNode::new("myOtherArgument")
            ))
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionArgumentNode::new("myArgument", NumberTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionArgumentNode","name":"myArgument","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionArgumentNode","name":"myArgument","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#;
        let node: InstructionArgumentNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionArgumentNode::new("myArgument", NumberTypeNode::le(U32))
        );
    }
}
