use crate::{EnumVariantDisplayNode, Text};

impl EnumVariantDisplayNode {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<Text>,
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
        assert_eq!(node.label, Some(Text::from("Buy")));
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
            label: Some("Buy".into()),
            skip_inner_data: Some(true),
            plugins: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumVariantDisplayNode","skipInnerData":true,"label":"Buy"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumVariantDisplayNode","label":"Buy"}"#;
        let node: EnumVariantDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, EnumVariantDisplayNode::new("Buy"));
    }
}
