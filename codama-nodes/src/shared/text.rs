use crate::TextNode;
use derive_more::derive::From;
use serde::{de, Deserialize, Serialize};

#[derive(From, Debug, PartialEq, Clone, Serialize)]
#[serde(untagged)]
pub enum Text {
    String(String),
    Node(TextNode),
}

impl Text {
    pub fn content(&self) -> &str {
        match self {
            Text::String(content) => content,
            Text::Node(node) => &node.content,
        }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text::String(value.to_string())
    }
}

// A derived untagged `Deserialize` hides why a `textNode` object fails, so this one
// dispatches on the JSON type and keeps the inner error.
impl<'de> Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TextVisitor;

        impl<'de> de::Visitor<'de> for TextVisitor {
            type Value = Text;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a textNode object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Text::from(value))
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                TextNode::deserialize(de::value::MapAccessDeserializer::new(map)).map(Text::Node)
            }
        }

        deserializer.deserialize_any(TextVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content() {
        assert_eq!(Text::from("Hello").content(), "Hello");
        let node = TextNode {
            content: "Hello".to_string(),
            plugins: vec![],
        };
        assert_eq!(Text::from(node).content(), "Hello");
    }

    #[test]
    fn to_json_string() {
        let json = serde_json::to_string(&Text::from("Hello")).unwrap();
        assert_eq!(json, r#""Hello""#);
    }

    #[test]
    fn to_json_node() {
        let text = Text::from(TextNode {
            content: "Hello".to_string(),
            plugins: vec![],
        });
        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(json, r#"{"kind":"textNode","content":"Hello"}"#);
    }

    #[test]
    fn from_json_string() {
        let text: Text = serde_json::from_str(r#""Hello""#).unwrap();
        assert_eq!(text, Text::from("Hello"));
    }

    #[test]
    fn from_json_node() {
        let text: Text = serde_json::from_str(r#"{"kind":"textNode","content":"Hello"}"#).unwrap();
        assert_eq!(
            text,
            Text::from(TextNode {
                content: "Hello".to_string(),
                plugins: vec![],
            })
        );
    }

    #[test]
    fn from_json_node_error_names_the_field() {
        let error = serde_json::from_str::<Text>(r#"{"kind":"textNode"}"#).unwrap_err();
        assert_eq!(
            error.to_string(),
            "missing field `content` at line 1 column 19"
        );
    }

    #[test]
    fn from_json_number_error() {
        let error = serde_json::from_str::<Text>("42").unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid type: integer `42`, expected a string or a textNode object at line 1 column 2"
        );
    }
}
