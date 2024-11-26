use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct EnumEmptyVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<usize>,
}

impl Into<crate::Node> for EnumEmptyVariantTypeNode {
    fn into(self) -> crate::Node {
        crate::Node::Type(self.into())
    }
}

impl EnumEmptyVariantTypeNode {
    pub fn new<T>(name: T, discriminator: Option<usize>) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            discriminator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = EnumEmptyVariantTypeNode::new("my_variant", Some(42));
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, Some(42));
    }

    #[test]
    fn to_json() {
        let node = EnumEmptyVariantTypeNode::new("myVariant", None);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumEmptyVariantTypeNode","name":"myVariant"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumEmptyVariantTypeNode","name":"myVariant"}"#;
        let node: EnumEmptyVariantTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, EnumEmptyVariantTypeNode::new("myVariant", None));
    }
}
