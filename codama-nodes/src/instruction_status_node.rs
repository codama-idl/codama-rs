use crate::{InstructionLifecycle, InstructionStatusNode, Text};

#[allow(clippy::derivable_impls)]
impl Default for InstructionStatusNode {
    fn default() -> Self {
        Self {
            lifecycle: InstructionLifecycle::default(),
            message: None,
            plugins: vec![],
        }
    }
}

impl InstructionStatusNode {
    pub fn new(lifecycle: InstructionLifecycle) -> Self {
        Self {
            lifecycle,
            message: None,
            plugins: vec![],
        }
    }

    pub fn with_message<S: Into<Text>>(lifecycle: InstructionLifecycle, message: S) -> Self {
        Self {
            lifecycle,
            message: Some(message.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let node = InstructionStatusNode::default();
        assert_eq!(node.lifecycle, InstructionLifecycle::Live);
        assert_eq!(node.message, None);
    }

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
        assert_eq!(node.message, Some(Text::from("Use newInstruction")));
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
        assert_eq!(node, InstructionStatusNode::new(InstructionLifecycle::Live));
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
        assert_eq!(
            node,
            InstructionStatusNode::with_message(
                InstructionLifecycle::Deprecated,
                "Use newInstruction instead"
            )
        );
    }
}
