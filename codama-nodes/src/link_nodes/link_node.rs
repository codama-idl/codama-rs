use crate::{
    AccountLinkNode, DefinedTypeLinkNode, InstructionAccountLinkNode, InstructionArgumentLinkNode,
    InstructionLinkNode, PdaLinkNode, ProgramLinkNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum LinkNode {
    Account(AccountLinkNode),
    DefinedType(DefinedTypeLinkNode),
    Instruction(InstructionLinkNode),
    InstructionAccount(InstructionAccountLinkNode),
    InstructionArgument(InstructionArgumentLinkNode),
    Pda(PdaLinkNode),
    Program(ProgramLinkNode),
}
