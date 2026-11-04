use crate::{Docs, Identifier, InstructionAccountNode, IsSigner};

impl InstructionAccountNode {
    pub fn new<T, U>(identifier: T, is_writable: bool, is_signer: U) -> Self
    where
        T: Into<Identifier>,
        U: Into<IsSigner>,
    {
        Self {
            identifier: identifier.into(),
            is_writable,
            is_signer: is_signer.into(),
            is_optional: None,
            docs: Docs::default(),
            default_value: Box::new(None),
            account_link: None,
            display: None,
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::id;
    use crate::{AccountValueNode, InstructionInputValueNode, Text};

    #[test]
    fn new() {
        let node = InstructionAccountNode::new(id("myAccount"), false, true);
        assert_eq!(node.identifier, id("myAccount"));
        assert!(!node.is_writable);
        assert_eq!(node.is_signer, IsSigner::True);
        assert_eq!(node.is_optional, None);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(*node.default_value, None);
    }

    #[test]
    fn direct_instantiation() {
        let node = InstructionAccountNode {
            identifier: id("myAccount"),
            is_writable: false,
            is_signer: IsSigner::Either,
            is_optional: Some(true),
            docs: vec!["Hello".to_string()].into(),
            default_value: Box::new(Some(AccountValueNode::new(id("myOtherAccount")).into())),
            account_link: None,
            display: None,
            plugins: vec![],
        };
        assert_eq!(node.identifier, id("myAccount"));
        assert!(!node.is_writable);
        assert_eq!(node.is_signer, IsSigner::Either);
        assert_eq!(node.is_optional, Some(true));
        assert_eq!(*node.docs, Some(Text::from("Hello")));
        assert_eq!(
            *node.default_value,
            Some(InstructionInputValueNode::AccountValue(
                AccountValueNode::new(id("myOtherAccount"))
            ))
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionAccountNode::new(id("myAccount"), false, true);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionAccountNode","identifier":"myAccount","isWritable":false,"isSigner":true}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionAccountNode","identifier":"myAccount","isWritable":false,"isSigner":true}"#;
        let node: InstructionAccountNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionAccountNode::new(id("myAccount"), false, true)
        );
    }
}
