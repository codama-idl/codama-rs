use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct ProgramLinkNode {
    // Data.
    pub name: CamelCaseString,
}

impl ProgramLinkNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self { name: name.into() }
    }
}

impl From<String> for ProgramLinkNode {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for ProgramLinkNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ProgramLinkNode::new("my_program");
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn from_string() {
        let node: ProgramLinkNode = String::from("my_program").into();
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn from_str() {
        let node: ProgramLinkNode = "my_program".into();
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
    }
}
