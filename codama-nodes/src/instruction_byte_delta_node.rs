use crate::{InstructionByteDeltaNode, InstructionByteDeltaValue};

impl InstructionByteDeltaNode {
    pub fn new<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaValue>,
    {
        Self {
            value: Box::new(value.into()),
            with_header,
            subtract: None,
        }
    }

    pub fn minus<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaValue>,
    {
        Self {
            value: Box::new(value.into()),
            with_header,
            subtract: Some(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ArgumentValueNode, InstructionByteDeltaNode, InstructionByteDeltaValue, NumberValueNode,
    };

    #[test]
    fn new() {
        let node = InstructionByteDeltaNode::new(ArgumentValueNode::new("myArgument"), true);
        assert_eq!(
            *node.value,
            InstructionByteDeltaValue::ArgumentValue(ArgumentValueNode::new("myArgument"))
        );
        assert!(node.with_header);
        assert_eq!(node.subtract, None);
    }

    #[test]
    fn minus() {
        let node = InstructionByteDeltaNode::minus(NumberValueNode::new(42), true);
        assert_eq!(
            *node.value,
            InstructionByteDeltaValue::NumberValue(NumberValueNode::new(42))
        );
        assert!(node.with_header);
        assert_eq!(node.subtract, Some(true));
    }

    #[test]
    fn to_json() {
        let node = InstructionByteDeltaNode::new(ArgumentValueNode::new("myArgument"), true);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionByteDeltaNode","withHeader":true,"value":{"kind":"argumentValueNode","name":"myArgument"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionByteDeltaNode","withHeader":true,"value":{"kind":"argumentValueNode","name":"myArgument"}}"#;
        let node: InstructionByteDeltaNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionByteDeltaNode::new(ArgumentValueNode::new("myArgument"), true)
        );
    }

    #[test]
    fn to_json_minus() {
        let node = InstructionByteDeltaNode::minus(ArgumentValueNode::new("myArgument"), true);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionByteDeltaNode","withHeader":true,"subtract":true,"value":{"kind":"argumentValueNode","name":"myArgument"}}"#
        );
    }

    #[test]
    fn from_json_minus() {
        let json = r#"{"kind":"instructionByteDeltaNode","withHeader":true,"subtract":true,"value":{"kind":"argumentValueNode","name":"myArgument"}}"#;
        let node: InstructionByteDeltaNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionByteDeltaNode::minus(ArgumentValueNode::new("myArgument"), true)
        );
    }
}
