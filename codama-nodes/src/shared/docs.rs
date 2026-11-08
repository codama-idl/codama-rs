use crate::Text;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Docs(Option<Text>);

impl Docs {
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

impl From<Text> for Docs {
    fn from(text: Text) -> Self {
        Self(Some(text))
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
    fn from_text() {
        let node = TextNode {
            content: "Hello".to_string(),
            plugins: vec![],
        };
        for text in [Text::from("Hello"), Text::from(node)] {
            assert_eq!(*Docs::from(text.clone()), Some(text));
        }
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
    fn from_vec_with_empty_string() {
        let docs: Docs = vec![String::new()].into();
        assert!(docs.is_empty());
    }

    #[test]
    fn empty_text_node() {
        let docs = Docs::from(Text::from(TextNode {
            content: String::new(),
            plugins: vec![],
        }));
        assert!(docs.is_empty());
    }

    #[test]
    fn empty_text_node_with_plugins() {
        let docs = Docs::from(Text::from(TextNode {
            content: String::new(),
            plugins: vec![PluginNode::new("anchor")],
        }));
        assert!(!docs.is_empty());
    }
}
