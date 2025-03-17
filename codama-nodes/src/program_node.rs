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
    pub defined_types: Vec<DefinedTypeNode>,
    pub pdas: Vec<PdaNode>,
    pub errors: Vec<ErrorNode>,
}

impl ProgramNode {
    pub fn new<T: Into<CamelCaseString>, U: Into<String>>(name: T, public_key: U) -> Self {
        Self {
            name: name.into(),
            public_key: public_key.into(),
            ..Default::default()
        }
    }

    pub fn set_version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = version.into();
        self
    }

    pub fn add_account(mut self, account: AccountNode) -> Self {
        self.accounts.push(account);
        self
    }

    pub fn add_instruction(mut self, instruction: InstructionNode) -> Self {
        self.instructions.push(instruction);
        self
    }

    pub fn add_defined_type(mut self, defined_type: DefinedTypeNode) -> Self {
        self.defined_types.push(defined_type);
        self
    }

    pub fn add_pda(mut self, pda: PdaNode) -> Self {
        self.pdas.push(pda);
        self
    }

    pub fn add_error(mut self, error: ErrorNode) -> Self {
        self.errors.push(error);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ProgramNode::new("my_program", "1234..5678");
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
        assert_eq!(node.public_key, "1234..5678".to_string());
        assert_eq!(node.version, "".to_string());
        assert_eq!(node.origin, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.accounts, vec![]);
        assert_eq!(node.instructions, vec![]);
        assert_eq!(node.defined_types, vec![]);
        assert_eq!(node.pdas, vec![]);
        assert_eq!(node.errors, vec![]);
    }

    #[test]
    fn default_program() {
        let node = ProgramNode::default();
        assert_eq!(node.name, CamelCaseString::new(""));
        assert_eq!(node.public_key, "".to_string());
        assert_eq!(node.version, "".to_string());
        assert_eq!(node.origin, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.accounts, vec![]);
        assert_eq!(node.instructions, vec![]);
        assert_eq!(node.defined_types, vec![]);
        assert_eq!(node.pdas, vec![]);
        assert_eq!(node.errors, vec![]);
    }

    #[test]
    fn direct_instantiation() {
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
