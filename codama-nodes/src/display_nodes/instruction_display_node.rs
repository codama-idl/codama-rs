use crate::InstructionDisplayNode;

impl InstructionDisplayNode {
    pub fn new<T>(intent: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            intent: Some(intent.into()),
            ..Default::default()
        }
    }

    pub fn interpolated<T>(interpolated_intent: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            interpolated_intent: Some(interpolated_intent.into()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionDisplayNode::new("Transfer");
        assert_eq!(node.intent.as_deref(), Some("Transfer"));
        assert_eq!(node.interpolated_intent, None);
    }

    #[test]
    fn interpolated() {
        let node = InstructionDisplayNode::interpolated("Send {amount} to {to}");
        assert_eq!(node.intent, None);
        assert_eq!(
            node.interpolated_intent.as_deref(),
            Some("Send {amount} to {to}")
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionDisplayNode {
            intent: Some("Transfer".to_string()),
            interpolated_intent: Some("Send {amount} to {to}".to_string()),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionDisplayNode","intent":"Transfer","interpolatedIntent":"Send {amount} to {to}"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionDisplayNode","intent":"Transfer"}"#;
        let node: InstructionDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionDisplayNode::new("Transfer"));
    }
}
