use crate::{
    AccountNode, ContextualValueNode, CountNode, DefinedTypeNode, DiscriminatorNode, ErrorNode,
    InstructionAccountNode, InstructionArgumentNode, InstructionByteDeltaNode, InstructionNode,
    InstructionRemainingAccountsNode, LinkNode, NodeTrait, NodeUnionTrait, PdaNode, PdaSeedNode,
    ProgramNode, RegisteredContextualValueNode, RegisteredTypeNode, RegisteredValueNode, RootNode,
    TypeNode, ValueNode,
};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

#[derive(From, Debug, PartialEq, Clone, Serialize, Deserialize)]
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

impl From<ContextualValueNode> for Node {
    fn from(node: ContextualValueNode) -> Self {
        Node::ContextualValue(node.into())
    }
}

impl From<TypeNode> for Node {
    fn from(node: TypeNode) -> Self {
        match node {
            TypeNode::Link(link) => link.into(),
            _ => Node::Type(node.try_into().unwrap()),
        }
    }
}

impl From<ValueNode> for Node {
    fn from(node: ValueNode) -> Self {
        Node::Value(node.into())
    }
}

impl NodeUnionTrait for Node {
    fn kind(&self) -> &'static str {
        match self {
            Node::ContextualValue(node) => node.kind(),
            Node::Count(node) => node.kind(),
            Node::Discriminator(node) => node.kind(),
            Node::Link(node) => node.kind(),
            Node::PdaSeed(node) => node.kind(),
            Node::Type(node) => node.kind(),
            Node::Value(node) => node.kind(),
            Node::Account(node) => node.kind(),
            Node::DefinedType(node) => node.kind(),
            Node::Error(node) => node.kind(),
            Node::Instruction(node) => node.kind(),
            Node::InstructionAccount(node) => node.kind(),
            Node::InstructionArgument(node) => node.kind(),
            Node::InstructionByteDelta(node) => node.kind(),
            Node::InstructionRemainingAccounts(node) => node.kind(),
            Node::Pda(node) => node.kind(),
            Node::Program(node) => node.kind(),
            Node::Root(node) => node.kind(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, U32};

    #[test]
    fn kind() {
        let node: Node = ProgramNode::default().into();
        assert_eq!(node.kind(), "programNode");
    }

    #[test]
    fn type_node_to_json() {
        let node: Node = NumberTypeNode::le(U32).into();
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
