use crate::{DisplaySkip, InstructionAccountDisplayNode};

impl InstructionAccountDisplayNode {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            label: Some(label.into()),
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
        let node = InstructionAccountDisplayNode::new("To");
        assert_eq!(node.label.as_deref(), Some("To"));
        assert_eq!(node.skip, None);
    }

    #[test]
    fn skipped() {
        let node = InstructionAccountDisplayNode::skipped(DisplaySkip::Always);
        assert_eq!(node.label, None);
        assert_eq!(node.skip, Some(DisplaySkip::Always));
    }

    #[test]
    fn to_json() {
        let node = InstructionAccountDisplayNode {
            label: Some("To".to_string()),
            skip: Some(DisplaySkip::WhenInjected),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionAccountDisplayNode","label":"To","skip":"whenInjected"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionAccountDisplayNode","label":"To"}"#;
        let node: InstructionAccountDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionAccountDisplayNode::new("To"));
    }
}
