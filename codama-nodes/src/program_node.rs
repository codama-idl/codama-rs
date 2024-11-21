use crate::{
    AccountNode, CamelCaseString, DefinedTypeNode, Docs, ErrorNode, InstructionNode, PdaNode,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramNode {
    // Data.
    pub name: CamelCaseString,
    pub public_key: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, // 'anchor' | 'shank'. Soon to be deprecated.
    #[serde(default)]
    #[serde(skip_serializing_if = "Docs::is_empty")]
    pub docs: Docs,

    // Children.
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub defined_types: Vec<DefinedTypeNode>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pdas: Vec<PdaNode>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
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

    #[test]
    fn to_json() {
        let node = ProgramNode {
            name: "myProgram".into(),
            public_key: "1234..5678".into(),
            version: "1.2.3".into(),
            ..ProgramNode::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"programNode","name":"myProgram","publicKey":"1234..5678","version":"1.2.3","accounts":[],"instructions":[]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"programNode","name":"myProgram","publicKey":"1234..5678","version":"1.2.3","accounts":[],"instructions":[]}"#;
        let node: ProgramNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ProgramNode {
                name: "myProgram".into(),
                public_key: "1234..5678".into(),
                version: "1.2.3".into(),
                ..ProgramNode::default()
            }
        );
    }
}
