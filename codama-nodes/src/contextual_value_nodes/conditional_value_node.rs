use crate::{
    AccountValueNode, ArgumentValueNode, InstructionInputValueNode, ResolverValueNode, ValueNode,
};
use codama_nodes_derive::{IntoEnum, Node};

#[derive(Node, Debug, PartialEq, Clone)]
pub struct ConditionalValueNode {
    // Children.
    pub condition: ConditionNode,
    pub value: Option<ValueNode>,
    pub if_true: Option<InstructionInputValueNode>,
    pub if_false: Option<InstructionInputValueNode>,
}

#[derive(IntoEnum, Debug, PartialEq, Clone)]
pub enum ConditionNode {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
    Resolver(ResolverValueNode),
}

#[cfg(test)]
mod tests {
    use crate::NumberValueNode;

    use super::*;

    #[test]
    fn direct_instantiation() {
        let node = ConditionalValueNode {
            condition: ArgumentValueNode::new("myArgument").into(),
            value: Some(NumberValueNode::new(42).into()),
            if_true: Some(AccountValueNode::new("myOtherAccount").into()),
            if_false: None,
        };
        assert_eq!(
            node.condition,
            ConditionNode::Argument(ArgumentValueNode::new("myArgument"))
        );
        assert_eq!(
            node.value,
            Some(ValueNode::Number(NumberValueNode::new(42)))
        );
        assert_eq!(
            node.if_true,
            Some(InstructionInputValueNode::Account(AccountValueNode::new(
                "myOtherAccount"
            )))
        );
        assert_eq!(node.if_false, None);
    }
}
