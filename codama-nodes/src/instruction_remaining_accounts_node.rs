use crate::{ArgumentValueNode, Docs, IsAccountSigner, ResolverValueNode};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct InstructionRemainingAccountsNode {
    // Data.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_optional: bool,
    pub is_signer: IsAccountSigner,
    pub is_writable: bool,
    #[serde(default, skip_serializing_if = "Docs::is_empty")]
    pub docs: Docs,

    // Children.
    pub value: InstructionRemainingAccountsNodeValue,
}

#[node_union]
pub enum InstructionRemainingAccountsNodeValue {
    Argument(ArgumentValueNode),
    Resolver(ResolverValueNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_instantiation() {
        let node = InstructionRemainingAccountsNode {
            is_optional: false,
            is_signer: IsAccountSigner::Either,
            is_writable: true,
            docs: vec!["This is a test".to_string()].into(),
            value: ArgumentValueNode::new("myArgument").into(),
        };
        assert!(!node.is_optional);
        assert_eq!(node.is_signer, IsAccountSigner::Either);
        assert!(node.is_writable);
        assert_eq!(node.docs, vec!["This is a test".to_string()].into());
        assert_eq!(
            node.value,
            InstructionRemainingAccountsNodeValue::Argument(ArgumentValueNode::new("myArgument"))
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionRemainingAccountsNode {
            is_optional: false,
            is_signer: IsAccountSigner::Either,
            is_writable: true,
            docs: vec![].into(),
            value: ArgumentValueNode::new("myArgument").into(),
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
                is_optional: false,
                is_signer: IsAccountSigner::Either,
                is_writable: true,
                docs: vec![].into(),
                value: ArgumentValueNode::new("myArgument").into(),
            }
        );
    }
}
