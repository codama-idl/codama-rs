use crate::{
    AccountLinkNode, CamelCaseString, DefinedTypeLinkNode, HasName, InstructionAccountLinkNode,
    InstructionArgumentLinkNode, InstructionLinkNode, PdaLinkNode, ProgramLinkNode,
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

impl HasName for LinkNode {
    fn name(&self) -> &CamelCaseString {
        match self {
            LinkNode::Account(node) => node.name(),
            LinkNode::DefinedType(node) => node.name(),
            LinkNode::Instruction(node) => node.name(),
            LinkNode::InstructionAccount(node) => node.name(),
            LinkNode::InstructionArgument(node) => node.name(),
            LinkNode::Pda(node) => node.name(),
            LinkNode::Program(node) => node.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasKind;

    #[test]
    fn kind() {
        let node: LinkNode = ProgramLinkNode::new("myProgram").into();
        assert_eq!(node.kind(), "programLinkNode");
    }
}
