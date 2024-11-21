use crate::{
    AccountValueNode, ArgumentValueNode, InstructionInputValueNode, ResolverValueNode, ValueNode,
};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct ConditionalValueNode {
    // Children.
    pub condition: ConditionNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_true: Option<InstructionInputValueNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_false: Option<InstructionInputValueNode>,
}

#[node_union]
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

    #[test]
    fn to_json() {
        let node = ConditionalValueNode {
            condition: ArgumentValueNode::new("myArgument").into(),
            value: Some(NumberValueNode::new(42).into()),
            if_true: Some(AccountValueNode::new("myOtherAccount").into()),
            if_false: None,
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"conditionalValueNode","condition":{"kind":"argumentValueNode","name":"myArgument"},"value":{"kind":"numberValueNode","number":42},"if_true":{"kind":"accountValueNode","name":"myOtherAccount"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"conditionalValueNode","condition":{"kind":"argumentValueNode","name":"myArgument"},"value":{"kind":"numberValueNode","number":42},"if_true":{"kind":"accountValueNode","name":"myOtherAccount"}}"#;
        let node: ConditionalValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConditionalValueNode {
                condition: ArgumentValueNode::new("myArgument").into(),
                value: Some(NumberValueNode::new(42u32).into()),
                if_true: Some(AccountValueNode::new("myOtherAccount").into()),
                if_false: None,
            }
        );
    }
}
