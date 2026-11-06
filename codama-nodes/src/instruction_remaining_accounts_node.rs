#[cfg(test)]
mod tests {
    use crate::{CamelCaseString, InstructionRemainingAccountsNode, IsSigner, Text};

    #[test]
    fn direct_instantiation() {
        let node = InstructionRemainingAccountsNode {
            identifier: "signers".into(),
            is_optional: None,
            is_signer: Some(IsSigner::Either),
            is_writable: Some(true),
            docs: vec!["This is a test".to_string()].into(),
            display: None,
            plugins: vec![],
        };
        assert_eq!(node.identifier, CamelCaseString::new("signers"));
        assert_eq!(node.is_optional, None);
        assert_eq!(node.is_signer, Some(IsSigner::Either));
        assert_eq!(node.is_writable, Some(true));
        assert_eq!(*node.docs, Some(Text::from("This is a test")));
        assert_eq!(node.display, None);
    }

    #[test]
    fn to_json_with_identifier_only() {
        let node = InstructionRemainingAccountsNode {
            identifier: "signers".into(),
            ..InstructionRemainingAccountsNode::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionRemainingAccountsNode","identifier":"signers"}"#
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionRemainingAccountsNode {
            identifier: "signers".into(),
            is_optional: None,
            is_signer: Some(IsSigner::Either),
            is_writable: Some(true),
            docs: vec![].into(),
            display: None,
            plugins: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionRemainingAccountsNode","identifier":"signers","isSigner":"either","isWritable":true}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionRemainingAccountsNode","identifier":"signers","isSigner":"either","isWritable":true}"#;
        let node: InstructionRemainingAccountsNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionRemainingAccountsNode {
                identifier: "signers".into(),
                is_optional: None,
                is_signer: Some(IsSigner::Either),
                is_writable: Some(true),
                docs: vec![].into(),
                display: None,
                plugins: vec![],
            }
        );
    }

    #[test]
    fn from_json_without_identifier() {
        let json = r#"{"kind":"instructionRemainingAccountsNode","isSigner":"either"}"#;
        let error = serde_json::from_str::<InstructionRemainingAccountsNode>(json).unwrap_err();
        assert!(error.to_string().starts_with("missing field `identifier`"));
    }
}
