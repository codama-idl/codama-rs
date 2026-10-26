use crate::{InstructionByteDeltaNode, InstructionByteDeltaValue};

impl InstructionByteDeltaNode {
    pub fn new<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaValue>,
    {
        Self {
            value: Box::new(value.into()),
            with_header,
            subtract: None,
            plugins: vec![],
        }
    }

    pub fn minus<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaValue>,
    {
        Self {
            value: Box::new(value.into()),
            with_header,
            subtract: Some(true),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AccountLinkNode, DataValueNode, InstructionByteDeltaNode, InstructionByteDeltaValue,
        IntegerValueNode, Path,
    };

    fn path(value: &str) -> Path {
        value.parse().unwrap()
    }

    #[test]
    fn new() {
        let node = InstructionByteDeltaNode::new(DataValueNode::new(path("myArgument")), true);
        assert_eq!(
            *node.value,
            InstructionByteDeltaValue::DataValue(DataValueNode::new(path("myArgument")))
        );
        assert!(node.with_header);
        assert_eq!(node.subtract, None);
    }

    #[test]
    fn new_with_account_link() {
        let node = InstructionByteDeltaNode::new(AccountLinkNode::new("myAccount"), false);
        assert_eq!(
            *node.value,
            InstructionByteDeltaValue::AccountLink(AccountLinkNode::new("myAccount"))
        );
        assert!(!node.with_header);
    }

    #[test]
    fn minus() {
        let node = InstructionByteDeltaNode::minus(IntegerValueNode::new(42), true);
        assert_eq!(
            *node.value,
            InstructionByteDeltaValue::IntegerValue(IntegerValueNode::new(42))
        );
        assert!(node.with_header);
        assert_eq!(node.subtract, Some(true));
    }

    #[test]
    fn to_json() {
        let node = InstructionByteDeltaNode::new(DataValueNode::new(path("myArgument")), true);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionByteDeltaNode","withHeader":true,"value":{"kind":"dataValueNode","path":"myArgument"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionByteDeltaNode","withHeader":true,"value":{"kind":"dataValueNode","path":"myArgument"}}"#;
        let node: InstructionByteDeltaNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionByteDeltaNode::new(DataValueNode::new(path("myArgument")), true)
        );
    }

    #[test]
    fn to_json_with_account_link() {
        let node = InstructionByteDeltaNode::new(AccountLinkNode::new("myAccount"), false);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionByteDeltaNode","withHeader":false,"value":{"kind":"accountLinkNode","identifier":"myAccount"}}"#
        );
    }

    #[test]
    fn from_json_with_account_link() {
        let json = r#"{"kind":"instructionByteDeltaNode","withHeader":false,"value":{"kind":"accountLinkNode","identifier":"myAccount"}}"#;
        let node: InstructionByteDeltaNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionByteDeltaNode::new(AccountLinkNode::new("myAccount"), false)
        );
    }

    #[test]
    fn to_json_minus() {
        let node = InstructionByteDeltaNode::minus(IntegerValueNode::new(42), true);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionByteDeltaNode","withHeader":true,"subtract":true,"value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json_minus() {
        let json = r#"{"kind":"instructionByteDeltaNode","withHeader":true,"subtract":true,"value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: InstructionByteDeltaNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionByteDeltaNode::minus(IntegerValueNode::new(42), true)
        );
    }
}
