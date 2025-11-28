use codama_nodes_derive::node;
use serde::{Deserialize, Serialize};

#[node]
#[derive(Default)]
pub struct InstructionStatusNode {
    // Data.
    pub status: InstructionStatus,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub message: String,
}

impl InstructionStatusNode {
    pub fn new(status: InstructionStatus) -> Self {
        Self {
            status,
            message: String::default(),
        }
    }

    pub fn with_message<S: Into<String>>(status: InstructionStatus, message: S) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InstructionStatus {
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
        let node = InstructionStatusNode::new(InstructionStatus::Live);
        assert_eq!(node.status, InstructionStatus::Live);
        assert_eq!(node.message, String::default());
    }

    #[test]
    fn with_message() {
        let node =
            InstructionStatusNode::with_message(InstructionStatus::Deprecated, "Use newInstruction");
        assert_eq!(node.status, InstructionStatus::Deprecated);
        assert_eq!(node.message, "Use newInstruction");
    }

    #[test]
    fn to_json() {
        let node = InstructionStatusNode::new(InstructionStatus::Live);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionStatusNode","status":"live"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionStatusNode","status":"live"}"#;
        let node: InstructionStatusNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.status, InstructionStatus::Live);
    }

    #[test]
    fn to_json_with_message() {
        let node = InstructionStatusNode::with_message(
            InstructionStatus::Deprecated,
            "Use newInstruction instead",
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionStatusNode","status":"deprecated","message":"Use newInstruction instead"}"#
        );
    }

    #[test]
    fn from_json_with_message() {
        let json =
            r#"{"kind":"instructionStatusNode","status":"deprecated","message":"Use newInstruction instead"}"#;
        let node: InstructionStatusNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.status, InstructionStatus::Deprecated);
        assert_eq!(node.message, "Use newInstruction instead");
    }
}
