use crate::{CamelCaseString, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct PdaLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub program: Option<ProgramLinkNode>,
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
}
