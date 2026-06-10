use crate::{InstructionLifecycle, InstructionStatusNode};

#[allow(clippy::derivable_impls)]
impl Default for InstructionStatusNode {
    fn default() -> Self {
        Self {
            lifecycle: InstructionLifecycle::default(),
            message: None,
        }
    }
}

impl InstructionStatusNode {
    pub fn new(lifecycle: InstructionLifecycle) -> Self {
        Self {
            lifecycle,
            message: None,
        }
    }

    pub fn with_message<S: Into<String>>(lifecycle: InstructionLifecycle, message: S) -> Self {
        Self {
            lifecycle,
            message: Some(message.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionStatusNode::new(InstructionLifecycle::Live);
        assert_eq!(node.lifecycle, InstructionLifecycle::Live);
        assert_eq!(node.message, None);
    }

    #[test]
    fn with_message() {
        let node = InstructionStatusNode::with_message(
            InstructionLifecycle::Deprecated,
            "Use newInstruction",
        );
        assert_eq!(node.lifecycle, InstructionLifecycle::Deprecated);
        assert_eq!(node.message.as_deref(), Some("Use newInstruction"));
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
        assert_eq!(node.message.as_deref(), Some("Use newInstruction instead"));
    }
}
