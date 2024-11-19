use crate::{CamelCaseString, Docs};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct ErrorNode {
    // Data.
    pub name: CamelCaseString,
    pub code: usize,
    pub message: String,
    pub docs: Docs,
}

impl ErrorNode {
    pub fn new<T, U>(name: T, code: usize, message: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<String>,
    {
        Self {
            name: name.into(),
            code,
            message: message.into(),
            docs: Docs::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ErrorNode::new("myError", 42, "Something went wrong");
        assert_eq!(node.name, CamelCaseString::new("myError"));
        assert_eq!(node.code, 42);
        assert_eq!(node.message, "Something went wrong".to_string());
        assert_eq!(node.docs, Docs::default());
    }

    #[test]
    fn direct_instantiation() {
        let node = ErrorNode {
            name: "myError".into(),
            code: 42,
            message: "Something went wrong".into(),
            docs: Docs::default(),
        };
        assert_eq!(node.name, CamelCaseString::new("myError"));
        assert_eq!(node.code, 42);
        assert_eq!(node.message, "Something went wrong".to_string());
        assert_eq!(node.docs, Docs::default());
    }
}
