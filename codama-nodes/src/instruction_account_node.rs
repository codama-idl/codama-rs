use crate::{CamelCaseString, Docs, InstructionAccountNode, IsSigner};

impl InstructionAccountNode {
    pub fn new<T, U>(name: T, is_writable: bool, is_signer: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<IsSigner>,
    {
        Self {
            name: name.into(),
            is_writable,
            is_signer: is_signer.into(),
            is_optional: None,
            docs: Docs::default(),
            default_value: Box::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountValueNode, InstructionInputValueNode};

    #[test]
    fn new() {
        let node = InstructionAccountNode::new("my_account", false, true);
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert!(!node.is_writable);
        assert_eq!(node.is_signer, IsSigner::True);
        assert_eq!(node.is_optional, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(*node.default_value, None);
    }

    #[test]
    fn direct_instantiation() {
        let node = InstructionAccountNode {
            name: "myAccount".into(),
            is_writable: false,
            is_signer: IsSigner::Either,
            is_optional: Some(true),
            docs: vec!["Hello".to_string()].into(),
            default_value: Box::new(Some(AccountValueNode::new("myOtherAccount").into())),
        };
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert!(!node.is_writable);
        assert_eq!(node.is_signer, IsSigner::Either);
        assert_eq!(node.is_optional, Some(true));
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(
            *node.default_value,
            Some(InstructionInputValueNode::AccountValue(
                AccountValueNode::new("myOtherAccount")
            ))
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionAccountNode::new("myAccount", false, true);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionAccountNode","name":"myAccount","isWritable":false,"isSigner":true}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionAccountNode","name":"myAccount","isWritable":false,"isSigner":true}"#;
        let node: InstructionAccountNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionAccountNode::new("myAccount", false, true));
    }
}
