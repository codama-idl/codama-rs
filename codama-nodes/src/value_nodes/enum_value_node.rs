use crate::{
    CamelCaseString, DefinedTypeLinkNode, EnumValueNode, StructValueNode, TupleValueNode, ValueNode,
};

impl EnumValueNode {
    pub fn new<T, U, V>(r#enum: T, variant: U, value: V) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
        V: Into<ValueNode>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: Box::new(Some(value.into())),
            plugins: vec![],
        }
    }

    pub fn empty<T, U>(r#enum: T, variant: U) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: Box::new(None),
            plugins: vec![],
        }
    }

    pub fn fields<T, U, V>(r#enum: T, variant: U, value: V) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
        V: Into<StructValueNode>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: Box::new(Some(ValueNode::Struct(value.into()))),
            plugins: vec![],
        }
    }

    pub fn tuple<T, U, V>(r#enum: T, variant: U, value: V) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
        V: Into<TupleValueNode>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: Box::new(Some(ValueNode::Tuple(value.into()))),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerValueNode, StringValueNode, StructFieldValueNode};

    #[test]
    fn new() {
        let node = EnumValueNode::new(
            "command",
            "move",
            StructValueNode::new(vec![
                StructFieldValueNode::new("x", IntegerValueNode::new(10)),
                StructFieldValueNode::new("y", IntegerValueNode::new(20)),
            ]),
        );
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, CamelCaseString::from("move"));
        assert_eq!(
            *node.value,
            Some(ValueNode::Struct(StructValueNode::new(vec![
                StructFieldValueNode::new("x", IntegerValueNode::new(10)),
                StructFieldValueNode::new("y", IntegerValueNode::new(20)),
            ])))
        );
    }

    #[test]
    fn new_with_any_value() {
        let node = EnumValueNode::new("command", "write", StringValueNode::new("Hello World"));
        assert_eq!(
            *node.value,
            Some(ValueNode::String(StringValueNode::new("Hello World")))
        );
    }

    #[test]
    fn empty() {
        let node = EnumValueNode::empty("command", "exit");
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, CamelCaseString::from("exit"));
        assert_eq!(*node.value, None);
    }

    #[test]
    fn fields() {
        let node = EnumValueNode::fields(
            "command",
            "move",
            vec![
                StructFieldValueNode::new("x", IntegerValueNode::new(10)),
                StructFieldValueNode::new("y", IntegerValueNode::new(20)),
            ],
        );
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, CamelCaseString::from("move"));
        assert_eq!(
            *node.value,
            Some(ValueNode::Struct(StructValueNode::new(vec![
                StructFieldValueNode::new("x", IntegerValueNode::new(10)),
                StructFieldValueNode::new("y", IntegerValueNode::new(20)),
            ])))
        );
    }

    #[test]
    fn tuple() {
        let node = EnumValueNode::tuple(
            "command",
            "write",
            vec![StringValueNode::new("Hello World").into()],
        );
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, CamelCaseString::from("write"));
        assert_eq!(
            *node.value,
            Some(ValueNode::Tuple(TupleValueNode::new(vec![
                StringValueNode::new("Hello World").into()
            ])))
        );
    }

    #[test]
    fn to_json() {
        let node = EnumValueNode::empty("command", "exit");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumValueNode","variant":"exit","enum":{"kind":"definedTypeLinkNode","identifier":"command"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumValueNode","variant":"exit","enum":{"kind":"definedTypeLinkNode","identifier":"command"}}"#;
        let node: EnumValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, EnumValueNode::empty("command", "exit"));
    }

    #[test]
    fn to_json_with_value() {
        let node = EnumValueNode::new("command", "move", IntegerValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumValueNode","variant":"move","enum":{"kind":"definedTypeLinkNode","identifier":"command"},"value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json_with_value() {
        let json = r#"{"kind":"enumValueNode","variant":"move","enum":{"kind":"definedTypeLinkNode","identifier":"command"},"value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: EnumValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            EnumValueNode::new("command", "move", IntegerValueNode::new(42))
        );
    }
}
