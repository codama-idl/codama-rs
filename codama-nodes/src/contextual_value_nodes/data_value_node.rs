use crate::{DataValueNode, Path};

impl DataValueNode {
    pub fn new<T>(path: T) -> Self
    where
        T: Into<Path>,
    {
        Self {
            path: path.into(),
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
        let node = DataValueNode::new(path("config.fees[0]"));
        assert_eq!(node.path.as_str(), "config.fees[0]");
    }

    #[test]
    fn to_json() {
        let node = DataValueNode::new(path("amount"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"dataValueNode","path":"amount"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"dataValueNode","path":"amount"}"#;
        let node: DataValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DataValueNode::new(path("amount")));
    }

    #[test]
    fn from_json_rejects_an_invalid_path() {
        let json = r#"{"kind":"dataValueNode","path":"my amount"}"#;
        let error = serde_json::from_str::<DataValueNode>(json).unwrap_err();
        assert!(error
            .to_string()
            .contains("expected a path expression string"));
    }
}
