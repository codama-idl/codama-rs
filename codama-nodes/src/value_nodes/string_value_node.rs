use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct StringValueNode {
    // Data.
    pub string: String,
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
}
