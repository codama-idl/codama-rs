#[cfg(test)]
mod tests {
    use crate::{
        AccountValueNode, ArgumentValueNode, ConditionalValueCondition, ConditionalValueNode,
        InstructionInputValueNode, NumberValueNode, ValueNode,
    };

    #[test]
    fn direct_instantiation() {
        let node = ConditionalValueNode {
            condition: Box::new(ArgumentValueNode::new("myArgument").into()),
            value: Box::new(Some(NumberValueNode::new(42).into())),
            if_true: Box::new(Some(AccountValueNode::new("myOtherAccount").into())),
            if_false: Box::new(None),
        };
        assert_eq!(
            *node.condition,
            ConditionalValueCondition::Argument(ArgumentValueNode::new("myArgument"))
        );
        assert_eq!(
            *node.value,
            Some(ValueNode::Number(NumberValueNode::new(42)))
        );
        assert_eq!(
            *node.if_true,
            Some(InstructionInputValueNode::AccountValue(
                AccountValueNode::new("myOtherAccount")
            ))
        );
        assert_eq!(*node.if_false, None);
    }

    #[test]
    fn to_json() {
        let node = ConditionalValueNode {
            condition: Box::new(ArgumentValueNode::new("myArgument").into()),
            value: Box::new(Some(NumberValueNode::new(42).into())),
            if_true: Box::new(Some(AccountValueNode::new("myOtherAccount").into())),
            if_false: Box::new(None),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"conditionalValueNode","condition":{"kind":"argumentValueNode","name":"myArgument"},"value":{"kind":"numberValueNode","number":42},"ifTrue":{"kind":"accountValueNode","name":"myOtherAccount"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"conditionalValueNode","condition":{"kind":"argumentValueNode","name":"myArgument"},"value":{"kind":"numberValueNode","number":42},"ifTrue":{"kind":"accountValueNode","name":"myOtherAccount"}}"#;
        let node: ConditionalValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConditionalValueNode {
                condition: Box::new(ArgumentValueNode::new("myArgument").into()),
                value: Box::new(Some(NumberValueNode::new(42u32).into())),
                if_true: Box::new(Some(AccountValueNode::new("myOtherAccount").into())),
                if_false: Box::new(None),
            }
        );
    }
}
