use crate::{CamelCaseString, HasName, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct PdaLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<PdaLinkNode> for crate::Node {
    fn from(val: PdaLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl PdaLinkNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            program: None,
        }
    }

    pub fn new_from_program<T>(name: T, program: ProgramLinkNode) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            program: Some(program),
        }
    }
}

impl From<String> for PdaLinkNode {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for PdaLinkNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl HasName for PdaLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PdaLinkNode::new("my_pda");
        assert_eq!(node.name, CamelCaseString::new("myPda"));
    }

    #[test]
    fn new_from_program() {
        let node = PdaLinkNode::new_from_program("my_pda", ProgramLinkNode::new("my_program"));
        assert_eq!(node.name, CamelCaseString::new("myPda"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }

    #[test]
    fn from_string() {
        let node: PdaLinkNode = String::from("my_pda").into();
        assert_eq!(node.name, CamelCaseString::new("myPda"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: PdaLinkNode = "my_pda".into();
        assert_eq!(node.name, CamelCaseString::new("myPda"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn to_json() {
        let node = PdaLinkNode::new("myPda");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"pdaLinkNode","name":"myPda"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"pdaLinkNode","name":"myPda"}"#;
        let node: PdaLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PdaLinkNode::new("myPda"));
    }
}
