use crate::{
    AccountNode, CamelCaseString, DefinedTypeNode, Docs, ErrorNode, InstructionNode, PdaNode,
};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone, Default)]
pub struct ProgramNode {
    // Data.
    pub name: CamelCaseString,
    pub public_key: String,
    pub version: String,
    pub origin: Option<String>, // 'anchor' | 'shank'. Soon to be deprecated.
    pub docs: Docs,

    // Children.
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    pub defined_types: Vec<DefinedTypeNode>,
    pub pdas: Vec<PdaNode>,
    pub errors: Vec<ErrorNode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program() {
        let node = ProgramNode {
            name: "myProgram".into(),
            public_key: "1234..5678".into(),
            version: "1.2.3".into(),
            ..ProgramNode::default()
        };
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
        assert_eq!(node.public_key, "1234..5678".to_string());
        assert_eq!(node.version, "1.2.3".to_string());
        assert_eq!(node.origin, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.accounts, vec![]);
        assert_eq!(node.instructions, vec![]);
        assert_eq!(node.defined_types, vec![]);
        assert_eq!(node.pdas, vec![]);
        assert_eq!(node.errors, vec![]);
    }
}
