use crate::{CamelCaseString, Docs, ErrorNode};

impl ErrorNode {
    pub fn new<T, U>(name: T, code: u32, message: U) -> Self
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

    #[test]
    fn to_json() {
        let node = ErrorNode::new("myError", 42, "Something went wrong");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"errorNode","name":"myError","code":42,"message":"Something went wrong"}"#
        );
    }

    #[test]
    fn from_json() {
        let json =
            r#"{"kind":"errorNode","name":"myError","code":42,"message":"Something went wrong"}"#;
        let node: ErrorNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ErrorNode::new("myError", 42, "Something went wrong"));
    }
}
