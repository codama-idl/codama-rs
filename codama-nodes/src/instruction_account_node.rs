use crate::{CamelCaseString, Docs, InstructionInputValueNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct InstructionAccountNode {
    // Data.
    pub name: CamelCaseString,
    pub is_writable: bool,
    pub is_signer: IsAccountSigner,
    pub is_optional: bool,
    pub docs: Docs,

    // Children.
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IsAccountSigner {
    True,
    False,
    Either,
}

impl From<bool> for IsAccountSigner {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
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
        assert_eq!(node.is_writable, false);
        assert_eq!(node.is_signer, IsAccountSigner::True);
        assert_eq!(node.is_optional, false);
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
        assert_eq!(node.is_writable, false);
        assert_eq!(node.is_signer, IsAccountSigner::Either);
        assert_eq!(node.is_optional, true);
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(
            node.default_value,
            Some(InstructionInputValueNode::Account(AccountValueNode::new(
                "myOtherAccount"
            )))
        );
    }
}
