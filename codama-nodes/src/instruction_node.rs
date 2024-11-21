use crate::{
    CamelCaseString, DiscriminatorNode, Docs, InstructionAccountNode, InstructionArgumentNode,
    InstructionByteDeltaNode, InstructionRemainingAccountsNode,
};
use codama_nodes_derive::node;
use serde::{Deserialize, Serialize};

#[node]
#[derive(Default)]
pub struct InstructionNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default)]
    #[serde(skip_serializing_if = "Docs::is_empty")]
    pub docs: Docs,
    pub optional_account_strategy: InstructionOptionalAccountStrategy,

    // Children.
    pub accounts: Vec<InstructionAccountNode>,
    pub arguments: Vec<InstructionArgumentNode>,
    pub extra_arguments: Vec<InstructionArgumentNode>,
    pub remaining_accounts: Vec<InstructionRemainingAccountsNode>,
    pub byte_deltas: Vec<InstructionByteDeltaNode>,
    pub discriminators: Vec<DiscriminatorNode>,
    pub sub_instructions: Vec<InstructionNode>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InstructionOptionalAccountStrategy {
    Omitted,
    #[default]
    ProgramId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_instruction() {
        let node = InstructionNode {
            name: "myInstruction".into(),
            ..InstructionNode::default()
        };
        assert_eq!(node.name, CamelCaseString::new("myInstruction"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(
            node.optional_account_strategy,
            InstructionOptionalAccountStrategy::ProgramId
        );
        assert_eq!(node.accounts, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.extra_arguments, vec![]);
        assert_eq!(node.remaining_accounts, vec![]);
        assert_eq!(node.byte_deltas, vec![]);
        assert_eq!(node.discriminators, vec![]);
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
}
