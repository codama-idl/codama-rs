use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct EnumEmptyVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<usize>,
}

impl From<EnumEmptyVariantTypeNode> for crate::Node {
    fn from(val: EnumEmptyVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl EnumEmptyVariantTypeNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            discriminator: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = EnumEmptyVariantTypeNode::new("my_variant");
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, None);
    }

    #[test]
    fn direct_instantiation() {
        let node = EnumEmptyVariantTypeNode {
            name: "my_variant".into(),
            discriminator: Some(42),
        };
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, Some(42));
    }

    #[test]
    fn to_json() {
        let node = EnumEmptyVariantTypeNode::new("myVariant");
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
        assert_eq!(node, EnumEmptyVariantTypeNode::new("myVariant"));
    }
}
