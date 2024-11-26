use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct FieldDiscriminatorNode {
    // Data.
    pub name: CamelCaseString,
    pub offset: usize,
}

impl Into<crate::Node> for FieldDiscriminatorNode {
    fn into(self) -> crate::Node {
        crate::Node::Discriminator(self.into())
    }
}

impl FieldDiscriminatorNode {
    pub fn new<T>(name: T, offset: usize) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = FieldDiscriminatorNode::new("my_field", 0);
        assert_eq!(node.name, CamelCaseString::new("myField"));
        assert_eq!(node.offset, 0);
    }

    #[test]
    fn to_json() {
        let node = FieldDiscriminatorNode::new("myField", 0);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"fieldDiscriminatorNode","name":"myField","offset":0}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"fieldDiscriminatorNode","name":"myField","offset":0}"#;
        let node: FieldDiscriminatorNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, FieldDiscriminatorNode::new("myField", 0));
    }
}
