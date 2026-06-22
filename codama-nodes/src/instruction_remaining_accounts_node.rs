// `InstructionRemainingAccountsNode` and its inline-union value type
// `InstructionRemainingAccountsValue` are generated; this file holds
// only the constructor-less node's tests.

#[cfg(test)]
mod tests {
    use crate::{
        ArgumentValueNode, InstructionRemainingAccountsNode, InstructionRemainingAccountsValue,
        IsSigner,
    };

    #[test]
    fn direct_instantiation() {
        let node = InstructionRemainingAccountsNode {
            is_optional: None,
            is_signer: Some(IsSigner::Either),
            is_writable: Some(true),
            docs: vec!["This is a test".to_string()].into(),
            value: Box::new(ArgumentValueNode::new("myArgument").into()),
            display: None,
        };
        assert_eq!(node.is_optional, None);
        assert_eq!(node.is_signer, Some(IsSigner::Either));
        assert_eq!(node.is_writable, Some(true));
        assert_eq!(node.docs, vec!["This is a test".to_string()].into());
        assert_eq!(
            *node.value,
            InstructionRemainingAccountsValue::Argument(ArgumentValueNode::new("myArgument"))
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionRemainingAccountsNode {
            is_optional: None,
            is_signer: Some(IsSigner::Either),
            is_writable: Some(true),
            docs: vec![].into(),
            value: Box::new(ArgumentValueNode::new("myArgument").into()),
            display: None,
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionRemainingAccountsNode","isSigner":"either","isWritable":true,"value":{"kind":"argumentValueNode","name":"myArgument"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionRemainingAccountsNode","isSigner":"either","isWritable":true,"value":{"kind":"argumentValueNode","name":"myArgument"}}"#;
        let node: InstructionRemainingAccountsNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionRemainingAccountsNode {
                is_optional: None,
                is_signer: Some(IsSigner::Either),
                is_writable: Some(true),
                docs: vec![].into(),
                value: Box::new(ArgumentValueNode::new("myArgument").into()),
                display: None,
            }
        );
    }
}
