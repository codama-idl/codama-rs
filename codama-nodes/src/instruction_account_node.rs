use crate::{CamelCaseString, Docs, HasName, InstructionInputValueNode, IsAccountSigner};
use codama_nodes_derive::node;

#[node]
pub struct InstructionAccountNode {
    // Data.
    pub name: CamelCaseString,
    pub is_writable: bool,
    pub is_signer: IsAccountSigner,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub is_optional: bool,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value: Option<InstructionInputValueNode>,
}

impl InstructionAccountNode {
    pub fn new<T, U>(name: T, is_writable: bool, is_signer: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<IsAccountSigner>,
    {
        Self {
            name: name.into(),
            is_writable,
            is_signer: is_signer.into(),
            is_optional: false,
            docs: Docs::default(),
            default_value: None,
        }
    }
}

impl HasName for InstructionAccountNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AccountValueNode;

    #[test]
    fn new() {
        let node = InstructionAccountNode::new("my_account", false, true);
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert!(!node.is_writable);
        assert_eq!(node.is_signer, IsAccountSigner::True);
        assert!(!node.is_optional);
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.default_value, None);
    }

    #[test]
    fn direct_instantiation() {
        let node = InstructionAccountNode {
            name: "myAccount".into(),
            is_writable: false,
            is_signer: IsAccountSigner::Either,
            is_optional: true,
            docs: vec!["Hello".to_string()].into(),
            default_value: Some(AccountValueNode::new("myOtherAccount").into()),
        };
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert!(!node.is_writable);
        assert_eq!(node.is_signer, IsAccountSigner::Either);
        assert!(node.is_optional);
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(
            node.default_value,
            Some(InstructionInputValueNode::Account(AccountValueNode::new(
                "myOtherAccount"
            )))
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
