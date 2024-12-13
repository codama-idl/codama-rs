use codama_nodes_derive::node;

#[node]
pub struct StringValueNode {
    // Data.
    pub string: String,
}

impl From<StringValueNode> for crate::Node {
    fn from(val: StringValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl StringValueNode {
    pub fn new<T>(string: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            string: string.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            StringValueNode::new("Hello World".to_string()).string,
            "Hello World".to_string()
        );
        assert_eq!(
            StringValueNode::new("Hello World").string,
            "Hello World".to_string()
        );
        assert_eq!(StringValueNode::new('a').string, "a".to_string());
    }

    #[test]
    fn to_json() {
        let node = StringValueNode::new("Hello World!");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"stringValueNode","string":"Hello World!"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"stringValueNode","string":"Hello World!"}"#;
        let node: StringValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, StringValueNode::new("Hello World!"));
    }
}
