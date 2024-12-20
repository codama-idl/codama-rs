use crate::{
    AccountLinkNode, DefinedTypeLinkNode, HasKind, InstructionAccountLinkNode,
    InstructionArgumentLinkNode, InstructionLinkNode, NodeUnionTrait, PdaLinkNode, ProgramLinkNode,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind() {
        let node: LinkNode = ProgramLinkNode::new("myProgram").into();
        assert_eq!(node.kind(), "programLinkNode");
    }
}
