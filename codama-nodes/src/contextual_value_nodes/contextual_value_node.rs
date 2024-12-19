use crate::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ConditionalValueNode,
    IdentityValueNode, NodeTrait, NodeUnionTrait, PayerValueNode, PdaSeedValueNode, PdaValueNode,
    ProgramIdValueNode, ResolverValueNode,
};
use codama_nodes_derive::{node_union, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredContextualValueNode {
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    Argument(ArgumentValueNode),
    Conditional(ConditionalValueNode),
    Identity(IdentityValueNode),
    Payer(PayerValueNode),
    Pda(PdaValueNode),
    ProgramId(ProgramIdValueNode),
    Resolver(ResolverValueNode),

    #[registered]
    PdaSeed(PdaSeedValueNode),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeUnionTrait;

    #[test]
    fn kind_from_standalone() {
        let node: ContextualValueNode = ProgramIdValueNode::new().into();
        assert_eq!(node.kind(), "programIdValueNode");
    }

    #[test]
    fn kind_from_registered() {
        let node: RegisteredContextualValueNode = ProgramIdValueNode::new().into();
        assert_eq!(node.kind(), "programIdValueNode");
    }
}
