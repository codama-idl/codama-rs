use crate::{CamelCaseString, Docs, ErrorNode, Text};

impl ErrorNode {
    pub fn new<T, U>(identifier: T, code: u32, message: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<Text>,
    {
        Self {
            identifier: identifier.into(),
            code,
            message: message.into(),
            docs: Docs::default(),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextNode;

    #[test]
    fn new() {
        let node = ErrorNode::new("myError", 42, "Something went wrong");
        assert_eq!(node.identifier, CamelCaseString::new("myError"));
        assert_eq!(node.code, 42);
        assert_eq!(node.message, Text::from("Something went wrong"));
        assert_eq!(node.docs, Docs::default());
    }

    #[test]
    fn direct_instantiation() {
        let node = ErrorNode {
            identifier: "myError".into(),
            code: 42,
            message: "Something went wrong".into(),
            docs: Docs::default(),
            plugins: vec![],
        };
        assert_eq!(node.identifier, CamelCaseString::new("myError"));
        assert_eq!(node.code, 42);
        assert_eq!(node.message, Text::from("Something went wrong"));
        assert_eq!(node.docs, Docs::default());
    }

    #[test]
    fn to_json() {
        let node = ErrorNode::new("myError", 42, "Something went wrong");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"errorNode","identifier":"myError","code":42,"message":"Something went wrong"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"errorNode","identifier":"myError","code":42,"message":"Something went wrong"}"#;
        let node: ErrorNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ErrorNode::new("myError", 42, "Something went wrong"));
    }

    #[test]
    fn to_json_with_text_node_message() {
        let message = TextNode {
            content: "Something went wrong".to_string(),
            plugins: vec![],
        };
        let node = ErrorNode::new("myError", 42, message);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"errorNode","identifier":"myError","code":42,"message":{"kind":"textNode","content":"Something went wrong"}}"#
        );
    }

    #[test]
    fn from_json_with_text_node_message() {
        let json = r#"{"kind":"errorNode","identifier":"myError","code":42,"message":{"kind":"textNode","content":"Something went wrong"}}"#;
        let node: ErrorNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node.message,
            Text::from(TextNode {
                content: "Something went wrong".to_string(),
                plugins: vec![],
            })
        );
    }
}
