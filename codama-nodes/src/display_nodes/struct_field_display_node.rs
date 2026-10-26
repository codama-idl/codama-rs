use crate::{DisplaySkip, StructFieldDisplayNode, Text};

impl StructFieldDisplayNode {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<Text>,
    {
        Self {
            label: Some(label.into()),
            ..Default::default()
        }
    }

    pub fn flattened() -> Self {
        Self {
            flatten: Some(true),
            ..Default::default()
        }
    }

    pub fn skipped(skip: DisplaySkip) -> Self {
        Self {
            skip: Some(skip),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerValueNode, TextNode, ValueLabelDisplayNode};

    #[test]
    fn new() {
        let node = StructFieldDisplayNode::new("Amount");
        assert_eq!(node.label, Some(Text::from("Amount")));
        assert_eq!(node.skip, None);
        assert_eq!(node.flatten, None);
        assert_eq!(node.flatten_prefix, None);
        assert_eq!(node.value_labels, vec![]);
    }

    #[test]
    fn flattened() {
        let node = StructFieldDisplayNode::flattened();
        assert_eq!(node.flatten, Some(true));
    }

    #[test]
    fn skipped() {
        let node = StructFieldDisplayNode::skipped(DisplaySkip::Always);
        assert_eq!(node.skip, Some(DisplaySkip::Always));
    }

    #[test]
    fn to_json() {
        let node = StructFieldDisplayNode {
            label: Some("Amount".into()),
            skip: Some(DisplaySkip::Never),
            flatten: Some(true),
            flatten_prefix: Some("inner".into()),
            value_labels: vec![ValueLabelDisplayNode::new(
                IntegerValueNode::new(0u8),
                "Zero",
            )],
            plugins: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldDisplayNode","skip":"never","flatten":true,"label":"Amount","flattenPrefix":"inner","valueLabels":[{"kind":"valueLabelDisplayNode","value":{"kind":"integerValueNode","value":"0"},"label":"Zero"}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structFieldDisplayNode","label":"Amount"}"#;
        let node: StructFieldDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, StructFieldDisplayNode::new("Amount"));
    }

    #[test]
    fn from_json_with_text_node_label() {
        let json =
            r#"{"kind":"structFieldDisplayNode","label":{"kind":"textNode","content":"Amount"}}"#;
        let node: StructFieldDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructFieldDisplayNode::new(TextNode {
                content: "Amount".to_string(),
                plugins: vec![],
            })
        );
    }
}
