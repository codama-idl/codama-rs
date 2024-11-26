use crate::{CamelCaseString, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct PdaSeedValueNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub value: ValueNode,
}

impl Into<crate::Node> for PdaSeedValueNode {
    fn into(self) -> crate::Node {
        crate::Node::ContextualValue(self.into())
    }
}

impl PdaSeedValueNode {
    pub fn new<T, U>(name: T, value: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<ValueNode>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NumberValueNode;

    use super::*;

    #[test]
    fn new() {
        let node = PdaSeedValueNode::new("answer", NumberValueNode::new(42));
        assert_eq!(node.name, CamelCaseString::from("answer"));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = PdaSeedValueNode::new("answer", NumberValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pdaSeedValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json: &str = r#"{"kind":"pdaSeedValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#;
        let node: PdaSeedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("answer", NumberValueNode::new(42u32))
        );
    }
}
