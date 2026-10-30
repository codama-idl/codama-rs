use crate::{
    AccountLinkNode, CamelCaseString, DefinedTypeLinkNode, HasIdentifier,
    InstructionAccountLinkNode, InstructionLinkNode, PdaLinkNode, ProgramLinkNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum LinkNode {
    Account(AccountLinkNode),
    DefinedType(DefinedTypeLinkNode),
    Instruction(InstructionLinkNode),
    InstructionAccount(InstructionAccountLinkNode),
    Pda(PdaLinkNode),
    Program(ProgramLinkNode),
}

impl HasIdentifier for LinkNode {
    fn identifier(&self) -> &CamelCaseString {
        match self {
            LinkNode::Account(node) => node.identifier(),
            LinkNode::DefinedType(node) => node.identifier(),
            LinkNode::Instruction(node) => node.identifier(),
            LinkNode::InstructionAccount(node) => node.identifier(),
            LinkNode::Pda(node) => node.identifier(),
            LinkNode::Program(node) => node.identifier(),
        }
    }
}
