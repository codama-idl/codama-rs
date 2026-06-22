use crate::EnumVariantDisplayNode;

impl EnumVariantDisplayNode {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            label: Some(label.into()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = EnumVariantDisplayNode::new("Buy");
        assert_eq!(node.label.as_deref(), Some("Buy"));
        assert_eq!(node.skip_inner_data, None);
    }

    #[test]
    fn default() {
        let node = EnumVariantDisplayNode::default();
        assert_eq!(node.label, None);
        assert_eq!(node.skip_inner_data, None);
    }

    #[test]
    fn to_json() {
        let node = EnumVariantDisplayNode {
            label: Some("Buy".to_string()),
            skip_inner_data: Some(true),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumVariantDisplayNode","label":"Buy","skipInnerData":true}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumVariantDisplayNode","label":"Buy"}"#;
        let node: EnumVariantDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, EnumVariantDisplayNode::new("Buy"));
    }
}
