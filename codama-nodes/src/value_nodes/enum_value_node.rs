use crate::{DefinedTypeLinkNode, EnumValueNode, Identifier, ValueNode};

impl EnumValueNode {
    /// A variant without data, such as `Command::Quit`.
    pub fn new<T, U>(r#enum: T, variant: U) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<Identifier>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: Box::new(None),
            plugins: vec![],
        }
    }

    /// A variant with data, such as a struct value for `Command::Move { x, y }`.
    pub fn with_value<T, U, V>(r#enum: T, variant: U, value: V) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<Identifier>,
        V: Into<ValueNode>,
    {
        Self {
            value: Box::new(Some(value.into())),
            ..Self::new(r#enum, variant)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerValueNode, StringValueNode, StructFieldValueNode, StructValueNode};

    #[test]
    fn with_value() {
        let node = EnumValueNode::with_value(
            DefinedTypeLinkNode::new("command"),
            "move",
            StructValueNode::new(vec![
                StructFieldValueNode::new("x", IntegerValueNode::new(10)),
                StructFieldValueNode::new("y", IntegerValueNode::new(20)),
            ]),
        );
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, Identifier::from("move"));
        assert_eq!(
            *node.value,
            Some(ValueNode::Struct(StructValueNode::new(vec![
                StructFieldValueNode::new("x", IntegerValueNode::new(10)),
                StructFieldValueNode::new("y", IntegerValueNode::new(20)),
            ])))
        );
    }

    #[test]
    fn with_any_value() {
        let node = EnumValueNode::with_value(
            DefinedTypeLinkNode::new("command"),
            "write",
            StringValueNode::new("Hello World"),
        );
        assert_eq!(
            *node.value,
            Some(ValueNode::String(StringValueNode::new("Hello World")))
        );
    }

    #[test]
    fn new() {
        let node = EnumValueNode::new(DefinedTypeLinkNode::new("command"), "exit");
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, Identifier::from("exit"));
        assert_eq!(*node.value, None);
    }

    #[test]
    fn to_json() {
        let node = EnumValueNode::new(DefinedTypeLinkNode::new("command"), "exit");
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
        assert_eq!(
            node,
            EnumValueNode::new(DefinedTypeLinkNode::new("command"), "exit")
        );
    }

    #[test]
    fn to_json_with_value() {
        let node = EnumValueNode::with_value(
            DefinedTypeLinkNode::new("command"),
            "move",
            IntegerValueNode::new(42),
        );
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
            EnumValueNode::with_value(
                DefinedTypeLinkNode::new("command"),
                "move",
                IntegerValueNode::new(42)
            )
        );
    }
}
