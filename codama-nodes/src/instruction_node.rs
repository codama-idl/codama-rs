// `InstructionNode` is generated; this file holds only its tests.

#[cfg(test)]
mod tests {
    use crate::{CamelCaseString, Docs, InstructionNode};

    #[test]
    fn empty_instruction() {
        let node = InstructionNode {
            name: "myInstruction".into(),
            ..InstructionNode::default()
        };
        assert_eq!(node.name, CamelCaseString::new("myInstruction"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.optional_account_strategy, None);
        assert_eq!(node.accounts, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.extra_arguments, vec![]);
        assert_eq!(node.remaining_accounts, vec![]);
        assert_eq!(node.byte_deltas, vec![]);
        assert_eq!(node.discriminators, vec![]);
        assert_eq!(node.status, None);
        assert_eq!(node.sub_instructions, vec![]);
    }

    #[test]
    fn instruction_with_sub_instructions() {
        let node = InstructionNode {
            name: "myInstruction".into(),
            sub_instructions: vec![
                InstructionNode {
                    name: "mySubInstructionA".into(),
                    ..InstructionNode::default()
                },
                InstructionNode {
                    name: "mySubInstructionB".into(),
                    ..InstructionNode::default()
                },
            ],
            ..InstructionNode::default()
        };
        assert_eq!(
            node.sub_instructions,
            vec![
                InstructionNode {
                    name: "mySubInstructionA".into(),
                    ..InstructionNode::default()
                },
                InstructionNode {
                    name: "mySubInstructionB".into(),
                    ..InstructionNode::default()
                },
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionNode {
            name: "myInstruction".into(),
            ..InstructionNode::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        // Empty arrays are omitted on write (skip-when-empty convention).
        assert_eq!(json, r#"{"kind":"instructionNode","name":"myInstruction"}"#);
    }

    #[test]
    fn from_json() {
        // Absent arrays default to empty on read (skip-when-empty convention).
        let json = r#"{"kind":"instructionNode","name":"myInstruction"}"#;
        let node: InstructionNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionNode {
                name: "myInstruction".into(),
                ..InstructionNode::default()
            }
        );
    }
}
