use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct ArgumentValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl Into<crate::Node> for ArgumentValueNode {
    fn into(self) -> crate::Node {
        crate::Node::ContextualValue(self.into())
    }
}

impl ArgumentValueNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self { name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ArgumentValueNode::new("my_argument");
        assert_eq!(node.name, CamelCaseString::new("myArgument"));
    }

    #[test]
    fn to_json() {
        let node = ArgumentValueNode::new("myArgument");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"argumentValueNode","name":"myArgument"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"argumentValueNode","name":"myArgument"}"#;
        let node: ArgumentValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ArgumentValueNode::new("myArgument"));
    }
}
