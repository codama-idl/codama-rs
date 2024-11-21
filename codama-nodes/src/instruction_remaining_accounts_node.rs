use crate::{ArgumentValueNode, Docs, IsAccountSigner, ResolverValueNode};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct InstructionRemainingAccountsNode {
    // Data.
    pub is_optional: bool,
    pub is_signer: IsAccountSigner,
    pub is_writable: bool,
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
        assert_eq!(node.is_optional, false);
        assert_eq!(node.is_signer, IsAccountSigner::Either);
        assert_eq!(node.is_writable, true);
        assert_eq!(node.docs, vec!["This is a test".to_string()].into());
        assert_eq!(
            node.value,
            InstructionRemainingAccountsNodeValue::Argument(ArgumentValueNode::new("myArgument"))
        );
    }
}
