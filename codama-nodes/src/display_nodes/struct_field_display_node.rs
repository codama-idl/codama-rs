use crate::{DisplaySkip, StructFieldDisplayNode};

impl StructFieldDisplayNode {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<String>,
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

    #[test]
    fn new() {
        let node = StructFieldDisplayNode::new("Amount");
        assert_eq!(node.label.as_deref(), Some("Amount"));
        assert_eq!(node.skip, None);
        assert_eq!(node.flatten, None);
        assert_eq!(node.flatten_prefix, None);
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
            label: Some("Amount".to_string()),
            skip: Some(DisplaySkip::Never),
            flatten: Some(true),
            flatten_prefix: Some("inner".to_string()),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldDisplayNode","label":"Amount","skip":"never","flatten":true,"flattenPrefix":"inner"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structFieldDisplayNode","label":"Amount"}"#;
        let node: StructFieldDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, StructFieldDisplayNode::new("Amount"));
    }
}
