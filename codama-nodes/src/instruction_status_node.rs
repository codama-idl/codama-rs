use codama_nodes_derive::node;
use serde::{Deserialize, Serialize};

#[node]
#[derive(Default)]
pub struct InstructionStatusNode {
    // Data.
    pub lifecycle: InstructionLifecycle,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub message: String,
}

impl InstructionStatusNode {
    pub fn new(lifecycle: InstructionLifecycle) -> Self {
        Self {
            lifecycle,
            message: String::default(),
        }
    }

    pub fn with_message<S: Into<String>>(lifecycle: InstructionLifecycle, message: S) -> Self {
        Self {
            lifecycle,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InstructionLifecycle {
    #[default]
    Live,
    Deprecated,
    Archived,
    Draft,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionStatusNode::new(InstructionLifecycle::Live);
        assert_eq!(node.lifecycle, InstructionLifecycle::Live);
        assert_eq!(node.message, String::default());
    }

    #[test]
    fn with_message() {
        let node = InstructionStatusNode::with_message(
            InstructionLifecycle::Deprecated,
            "Use newInstruction",
        );
        assert_eq!(node.lifecycle, InstructionLifecycle::Deprecated);
        assert_eq!(node.message, "Use newInstruction");
    }

    #[test]
    fn to_json() {
        let node = InstructionStatusNode::new(InstructionLifecycle::Live);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionStatusNode","lifecycle":"live"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionStatusNode","lifecycle":"live"}"#;
        let node: InstructionStatusNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.lifecycle, InstructionLifecycle::Live);
    }

    #[test]
    fn to_json_with_message() {
        let node = InstructionStatusNode::with_message(
            InstructionLifecycle::Deprecated,
            "Use newInstruction instead",
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionStatusNode","lifecycle":"deprecated","message":"Use newInstruction instead"}"#
        );
    }

    #[test]
    fn from_json_with_message() {
        let json = r#"{"kind":"instructionStatusNode","lifecycle":"deprecated","message":"Use newInstruction instead"}"#;
        let node: InstructionStatusNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.lifecycle, InstructionLifecycle::Deprecated);
        assert_eq!(node.message, "Use newInstruction instead");
    }
}
