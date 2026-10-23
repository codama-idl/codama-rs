use crate::Text;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Docs(Option<Text>);

impl Docs {
    pub fn push<T: Into<String>>(&mut self, line: T) {
        let line = line.into();
        match &mut self.0 {
            None => self.0 = Some(Text::String(line)),
            Some(text) => {
                let content = match text {
                    Text::String(content) => content,
                    Text::Node(node) => &mut node.content,
                };
                if !content.is_empty() {
                    content.push('\n');
                }
                content.push_str(&line);
            }
        }
    }

    /// Empty text counts as no docs, so callers that merge docs can replace it.
    /// The text still serializes as written.
    pub fn is_empty(&self) -> bool {
        match &self.0 {
            None => true,
            Some(Text::String(content)) => content.is_empty(),
            Some(Text::Node(node)) => node.content.is_empty() && node.plugins.is_empty(),
        }
    }
}

impl From<Vec<String>> for Docs {
    fn from(lines: Vec<String>) -> Self {
        if lines.is_empty() {
            return Self::default();
        }
        Self(Some(Text::String(lines.join("\n"))))
    }
}

impl Deref for Docs {
    type Target = Option<Text>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PluginNode, TextNode};

    #[test]
    fn default() {
        let docs = Docs::default();
        assert!(docs.is_empty());
        assert_eq!(*docs, None);
    }

    #[test]
    fn push() {
        let mut docs = Docs::default();
        docs.push("Hello");
        docs.push(String::from("World"));
        assert_eq!(*docs, Some(Text::from("Hello\nWorld")));
    }

    #[test]
    fn push_to_text_node() {
        let mut docs = Docs(Some(Text::from(TextNode {
            content: "Hello".to_string(),
            plugins: vec![],
        })));
        docs.push("World");
        assert_eq!(docs.as_ref().map(Text::content), Some("Hello\nWorld"));
    }

    #[test]
    fn from_vec() {
        let docs: Docs = vec!["Hello".to_string(), "World".to_string()].into();
        assert_eq!(*docs, Some(Text::from("Hello\nWorld")));
    }

    #[test]
    fn from_empty_vec() {
        let docs: Docs = vec![].into();
        assert!(docs.is_empty());
    }

    #[test]
    fn to_json() {
        let docs: Docs = vec!["Hello".to_string(), "World".to_string()].into();
        let json = serde_json::to_string(&docs).unwrap();
        assert_eq!(json, r#""Hello\nWorld""#);
    }

    #[test]
    fn from_json() {
        let docs: Docs = serde_json::from_str(r#""Hello\nWorld""#).unwrap();
        assert_eq!(docs, vec!["Hello".to_string(), "World".to_string()].into());
    }

    #[test]
    fn from_empty_json() {
        let docs: Docs = serde_json::from_str(r#""""#).unwrap();
        assert!(docs.is_empty());
        let json = serde_json::to_string(&docs).unwrap();
        assert_eq!(json, r#""""#);
    }

    #[test]
    fn push_to_empty_json() {
        let mut docs: Docs = serde_json::from_str(r#""""#).unwrap();
        docs.push("Hello");
        assert_eq!(*docs, Some(Text::from("Hello")));
    }

    #[test]
    fn from_vec_with_empty_string() {
        let docs: Docs = vec![String::new()].into();
        assert!(docs.is_empty());
    }

    #[test]
    fn empty_text_node() {
        let docs = Docs(Some(Text::from(TextNode {
            content: String::new(),
            plugins: vec![],
        })));
        assert!(docs.is_empty());
    }

    #[test]
    fn empty_text_node_with_plugins() {
        let docs = Docs(Some(Text::from(TextNode {
            content: String::new(),
            plugins: vec![PluginNode {
                namespace: "anchor".to_string(),
                ..PluginNode::default()
            }],
        })));
        assert!(!docs.is_empty());
    }
}
