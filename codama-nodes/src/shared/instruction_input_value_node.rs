use crate::{HasKind, InstructionInputValueNode, ValueNode};
use codama_errors::CodamaError;

/// Bridge from the value-node union into the broader
/// `InstructionInputValueNode`. The generated `InstructionInputValueNode`
/// enum (in `generated/contextual_value_nodes/`) auto-derives
/// `From<Variant>` for each leaf, but `ValueNode` itself (a union of
/// 14 value leaves) needs this explicit bridge.
impl From<ValueNode> for InstructionInputValueNode {
    fn from(value: ValueNode) -> Self {
        match value {
            ValueNode::Array(value) => Self::ArrayValue(value),
            ValueNode::Boolean(value) => Self::BooleanValue(value),
            ValueNode::Bytes(value) => Self::BytesValue(value),
            ValueNode::Constant(value) => Self::ConstantValue(value),
            ValueNode::Enum(value) => Self::EnumValue(value),
            ValueNode::Map(value) => Self::MapValue(value),
            ValueNode::None(value) => Self::NoneValue(value),
            ValueNode::Number(value) => Self::NumberValue(value),
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
            InstructionInputValueNode::MapValue(value) => Ok(Self::Map(value)),
            InstructionInputValueNode::NoneValue(value) => Ok(Self::None(value)),
            InstructionInputValueNode::NumberValue(value) => Ok(Self::Number(value)),
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
