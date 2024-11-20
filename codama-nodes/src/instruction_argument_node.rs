use crate::{CamelCaseString, DefaultValueStrategy, Docs, InstructionInputValueNode, TypeNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone)]
pub struct InstructionArgumentNode {
    // Data.
    pub name: CamelCaseString,
    pub default_value_strategy: Option<DefaultValueStrategy>,
    pub docs: Docs,

    // Children.
    pub r#type: TypeNode,
    pub default_value: Option<InstructionInputValueNode>,
}

impl InstructionArgumentNode {
    pub fn new<T, U>(name: T, r#type: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            name: name.into(),
            default_value_strategy: None,
            docs: Docs::default(),
            r#type: r#type.into(),
            default_value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArgumentValueNode, NumberTypeNode, U32};

    #[test]
    fn new() {
        let node = InstructionArgumentNode::new("my_argument", NumberTypeNode::le(U32));
        assert_eq!(node.name, CamelCaseString::new("myArgument"));
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
    }

    #[test]
    fn direct_instantiation() {
        let node = InstructionArgumentNode {
            name: "myArgument".into(),
            default_value_strategy: Some(DefaultValueStrategy::Optional),
            docs: vec!["Hello".to_string()].into(),
            r#type: NumberTypeNode::le(U32).into(),
            default_value: Some(ArgumentValueNode::new("myOtherArgument").into()),
        };

        assert_eq!(node.name, CamelCaseString::new("myArgument"));
        assert_eq!(
            node.default_value_strategy,
            Some(DefaultValueStrategy::Optional)
        );
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
        assert_eq!(
            node.default_value,
            Some(InstructionInputValueNode::Argument(ArgumentValueNode::new(
                "myOtherArgument"
            )))
        );
    }
}
