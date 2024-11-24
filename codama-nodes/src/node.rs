use crate::{
    AccountNode, CountNode, DefinedTypeNode, DiscriminatorNode, ErrorNode, InstructionAccountNode,
    InstructionArgumentNode, InstructionByteDeltaNode, InstructionNode,
    InstructionRemainingAccountsNode, LinkNode, PdaNode, PdaSeedNode, ProgramNode,
    RegisteredContextualValueNode, RegisteredTypeNode, RegisteredValueNode, RootNode,
};
use codama_nodes_derive::IntoEnum;
use serde::{Deserialize, Serialize};

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
    // Node unions.
    ContextualValue(RegisteredContextualValueNode),
    Count(CountNode),
    Discriminator(DiscriminatorNode),
    Link(LinkNode),
    PdaSeed(PdaSeedNode),
    Type(RegisteredTypeNode),
    Value(RegisteredValueNode),

    // Nodes.
    Account(AccountNode),
    DefinedType(DefinedTypeNode),
    Error(ErrorNode),
    Instruction(InstructionNode),
    InstructionAccount(InstructionAccountNode),
    InstructionArgument(InstructionArgumentNode),
    InstructionByteDelta(InstructionByteDeltaNode),
    InstructionRemainingAccounts(InstructionRemainingAccountsNode),
    Pda(PdaNode),
    Program(ProgramNode),
    Root(RootNode),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, U32};

    #[test]
    fn type_node_to_json() {
        let node: Node = RegisteredTypeNode::Number(NumberTypeNode::le(U32)).into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"numberTypeNode","format":"u32","endian":"le"}"#
        );
    }

    #[test]
    fn type_node_from_json() {
        let json = r#"{"kind":"numberTypeNode","format":"u32","endian":"le"}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(U32)))
        );
    }

    #[test]
    fn defined_type_to_json() {
        let node: Node = DefinedTypeNode::new("myType", NumberTypeNode::le(U32)).into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"definedTypeNode","name":"myType","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn defined_type_from_json() {
        let json = r#"{"kind":"definedTypeNode","name":"myType","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            Node::DefinedType(DefinedTypeNode::new("myType", NumberTypeNode::le(U32)))
        );
    }
}
