use crate::{FieldDiscriminatorNode, Path};

impl FieldDiscriminatorNode {
    pub fn new<T>(path: T, offset: u64) -> Self
    where
        T: Into<Path>,
    {
        Self {
            path: path.into(),
            offset,
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(value: &str) -> Path {
        value.parse().unwrap()
    }

    #[test]
    fn new() {
        let node = FieldDiscriminatorNode::new(path("header.kind"), 4);
        assert_eq!(node.path.as_str(), "header.kind");
        assert_eq!(node.offset, 4);
    }

    #[test]
    fn to_json() {
        let node = FieldDiscriminatorNode::new(path("myField"), 0);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"fieldDiscriminatorNode","path":"myField","offset":0}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"fieldDiscriminatorNode","path":"myField","offset":0}"#;
        let node: FieldDiscriminatorNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, FieldDiscriminatorNode::new(path("myField"), 0));
    }

    #[test]
    fn from_json_rejects_an_invalid_path() {
        let json = r#"{"kind":"fieldDiscriminatorNode","path":"my field","offset":0}"#;
        let error = serde_json::from_str::<FieldDiscriminatorNode>(json).unwrap_err();
        assert!(error
            .to_string()
            .contains("expected a path expression string"));
    }
}
