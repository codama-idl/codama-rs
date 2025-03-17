use crate::{AccountLinkNode, ArgumentValueNode, NumberValueNode, ResolverValueNode};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct InstructionByteDeltaNode {
    // Data.
    pub with_header: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::is_default")]
    pub subtract: bool,

    // Children.
    pub value: InstructionByteDeltaNodeValue,
}

impl InstructionByteDeltaNode {
    pub fn new<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaNodeValue>,
    {
        Self {
            value: value.into(),
            with_header,
            subtract: false,
        }
    }

    pub fn minus<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaNodeValue>,
    {
        Self {
            value: value.into(),
            with_header,
            subtract: true,
        }
    }
}

#[node_union]
pub enum InstructionByteDeltaNodeValue {
    Account(AccountLinkNode),
    Argument(ArgumentValueNode),
    Number(NumberValueNode),
    Resolver(ResolverValueNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionByteDeltaNode::new(ArgumentValueNode::new("myArgument"), true);
        assert_eq!(
            node.value,
            InstructionByteDeltaNodeValue::Argument(ArgumentValueNode::new("myArgument"))
        );
        assert!(node.with_header);
        assert!(!node.subtract);
    }

    #[test]
    fn minus() {
        let node = InstructionByteDeltaNode::minus(NumberValueNode::new(42), true);
        assert_eq!(
            node.value,
            InstructionByteDeltaNodeValue::Number(NumberValueNode::new(42))
        );
        assert!(node.with_header);
        assert!(node.subtract);
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
