use crate::{HasKind, InstructionInputValueNode, Node, ValueNode};
use codama_errors::CodamaError;

/// Bridge from the value-node union into the broader
/// `InstructionInputValueNode`. The generated `InstructionInputValueNode`
/// enum (in `generated/contextual_value_nodes/`) auto-derives
/// `From<Variant>` for each leaf, but `ValueNode` itself (a union of
/// 16 value leaves) needs this explicit bridge.
impl From<ValueNode> for InstructionInputValueNode {
    fn from(value: ValueNode) -> Self {
        match value {
            ValueNode::Array(value) => Self::ArrayValue(value),
            ValueNode::Boolean(value) => Self::BooleanValue(value),
            ValueNode::Bytes(value) => Self::BytesValue(value),
            ValueNode::Constant(value) => Self::ConstantValue(value),
            ValueNode::Enum(value) => Self::EnumValue(value),
            ValueNode::Float(value) => Self::FloatValue(value),
            ValueNode::Injected(value) => Self::InjectedValue(value),
            ValueNode::Integer(value) => Self::IntegerValue(value),
            ValueNode::Map(value) => Self::MapValue(value),
            ValueNode::None(value) => Self::NoneValue(value),
            ValueNode::PublicKey(value) => Self::PublicKeyValue(value),
            ValueNode::Set(value) => Self::SetValue(value),
            ValueNode::Some(value) => Self::SomeValue(value),
            ValueNode::String(value) => Self::StringValue(value),
            ValueNode::Struct(value) => Self::StructValue(value),
            ValueNode::Tuple(value) => Self::TupleValue(value),
        }
    }
}

/// Inverse bridge: extract the `ValueNode` subset out of
/// `InstructionInputValueNode`, returning `CodamaError::InvalidNodeConversion`
/// for the non-value variants (contextual / link members).
impl TryFrom<InstructionInputValueNode> for ValueNode {
    type Error = CodamaError;

    fn try_from(value: InstructionInputValueNode) -> Result<Self, Self::Error> {
        match value {
            InstructionInputValueNode::ArrayValue(value) => Ok(Self::Array(value)),
            InstructionInputValueNode::BooleanValue(value) => Ok(Self::Boolean(value)),
            InstructionInputValueNode::BytesValue(value) => Ok(Self::Bytes(value)),
            InstructionInputValueNode::ConstantValue(value) => Ok(Self::Constant(value)),
            InstructionInputValueNode::EnumValue(value) => Ok(Self::Enum(value)),
            InstructionInputValueNode::FloatValue(value) => Ok(Self::Float(value)),
            InstructionInputValueNode::InjectedValue(value) => Ok(Self::Injected(value)),
            InstructionInputValueNode::IntegerValue(value) => Ok(Self::Integer(value)),
            InstructionInputValueNode::MapValue(value) => Ok(Self::Map(value)),
            InstructionInputValueNode::NoneValue(value) => Ok(Self::None(value)),
            InstructionInputValueNode::PublicKeyValue(value) => Ok(Self::PublicKey(value)),
            InstructionInputValueNode::SetValue(value) => Ok(Self::Set(value)),
            InstructionInputValueNode::SomeValue(value) => Ok(Self::Some(value)),
            InstructionInputValueNode::StringValue(value) => Ok(Self::String(value)),
            InstructionInputValueNode::StructValue(value) => Ok(Self::Struct(value)),
            InstructionInputValueNode::TupleValue(value) => Ok(Self::Tuple(value)),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: value.kind().to_string(),
                into: "ValueNode".to_string(),
            }),
        }
    }
}

/// Bridge into `Node`, for slots that take any node such as `providedNode.node`.
impl From<InstructionInputValueNode> for Node {
    fn from(value: InstructionInputValueNode) -> Self {
        match value {
            InstructionInputValueNode::AccountBumpValue(value) => value.into(),
            InstructionInputValueNode::AccountDataValue(value) => value.into(),
            InstructionInputValueNode::AccountValue(value) => value.into(),
            InstructionInputValueNode::ArrayValue(value) => value.into(),
            InstructionInputValueNode::BooleanValue(value) => value.into(),
            InstructionInputValueNode::BytesValue(value) => value.into(),
            InstructionInputValueNode::ConditionalValue(value) => value.into(),
            InstructionInputValueNode::ConstantValue(value) => value.into(),
            InstructionInputValueNode::DataValue(value) => value.into(),
            InstructionInputValueNode::EnumValue(value) => value.into(),
            InstructionInputValueNode::FloatValue(value) => value.into(),
            InstructionInputValueNode::IdentityValue(value) => value.into(),
            InstructionInputValueNode::InjectedValue(value) => value.into(),
            InstructionInputValueNode::IntegerValue(value) => value.into(),
            InstructionInputValueNode::MapValue(value) => value.into(),
            InstructionInputValueNode::NoneValue(value) => value.into(),
            InstructionInputValueNode::PayerValue(value) => value.into(),
            InstructionInputValueNode::PdaValue(value) => value.into(),
            InstructionInputValueNode::ProgramIdValue(value) => value.into(),
            InstructionInputValueNode::ProgramLink(value) => value.into(),
            InstructionInputValueNode::PublicKeyValue(value) => value.into(),
            InstructionInputValueNode::SetValue(value) => value.into(),
            InstructionInputValueNode::SomeValue(value) => value.into(),
            InstructionInputValueNode::StringValue(value) => value.into(),
            InstructionInputValueNode::StructValue(value) => value.into(),
            InstructionInputValueNode::TupleValue(value) => value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        IntegerValueNode, LinkNode, PayerValueNode, ProgramLinkNode, RegisteredContextualValueNode,
        RegisteredValueNode,
    };

    #[test]
    fn into_node() {
        for (value, expected) in [
            (
                InstructionInputValueNode::from(PayerValueNode::new()),
                Node::ContextualValue(RegisteredContextualValueNode::Payer(PayerValueNode::new())),
            ),
            (
                IntegerValueNode::new(42).into(),
                Node::Value(RegisteredValueNode::Integer(IntegerValueNode::new(42))),
            ),
            (
                ProgramLinkNode::new("system").into(),
                Node::Link(LinkNode::Program(ProgramLinkNode::new("system"))),
            ),
        ] {
            assert_eq!(Node::from(value), expected);
        }
    }
}
