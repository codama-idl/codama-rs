#[cfg(test)]
mod tests {
    use crate::{
        AccountValueNode, ConditionalValueCondition, ConditionalValueNode, DataValueNode,
        InstructionInputValueNode, IntegerValueNode, Path, ValueNode,
    };

    fn path(value: &str) -> Path {
        value.parse().unwrap()
    }

    #[test]
    fn direct_instantiation() {
        let node = ConditionalValueNode {
            condition: Box::new(DataValueNode::new(path("myField")).into()),
            value: Box::new(Some(IntegerValueNode::new(42).into())),
            if_true: Box::new(Some(AccountValueNode::new("myOtherAccount").into())),
            if_false: Box::new(None),
            plugins: vec![],
        };
        assert_eq!(
            *node.condition,
            ConditionalValueCondition::Data(DataValueNode::new(path("myField")))
        );
        assert_eq!(
            *node.value,
            Some(ValueNode::Integer(IntegerValueNode::new(42)))
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
            condition: Box::new(DataValueNode::new(path("myField")).into()),
            value: Box::new(Some(IntegerValueNode::new(42).into())),
            if_true: Box::new(Some(AccountValueNode::new("myOtherAccount").into())),
            if_false: Box::new(None),
            plugins: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"conditionalValueNode","condition":{"kind":"dataValueNode","path":"myField"},"value":{"kind":"integerValueNode","value":"42"},"ifTrue":{"kind":"accountValueNode","identifier":"myOtherAccount"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"conditionalValueNode","condition":{"kind":"dataValueNode","path":"myField"},"value":{"kind":"integerValueNode","value":"42"},"ifTrue":{"kind":"accountValueNode","identifier":"myOtherAccount"}}"#;
        let node: ConditionalValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConditionalValueNode {
                condition: Box::new(DataValueNode::new(path("myField")).into()),
                value: Box::new(Some(IntegerValueNode::new(42u32).into())),
                if_true: Box::new(Some(AccountValueNode::new("myOtherAccount").into())),
                if_false: Box::new(None),
                plugins: vec![],
            }
        );
    }
}
