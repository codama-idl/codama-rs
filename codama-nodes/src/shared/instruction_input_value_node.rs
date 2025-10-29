use crate::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ArrayValueNode, BooleanValueNode,
    BytesValueNode, ConditionalValueNode, ConstantValueNode, EnumValueNode, HasKind,
    IdentityValueNode, MapValueNode, NoneValueNode, NumberValueNode, PayerValueNode, PdaValueNode,
    ProgramIdValueNode, ProgramLinkNode, PublicKeyValueNode, ResolverValueNode, SetValueNode,
    SomeValueNode, StringValueNode, StructValueNode, TupleValueNode, ValueNode,
};
use codama_errors::CodamaError;
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionInputValueNode {
    // ContextualValueNodes.
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    Argument(ArgumentValueNode),
    Conditional(ConditionalValueNode),
    Identity(IdentityValueNode),
    Payer(PayerValueNode),
    Pda(PdaValueNode),
    ProgramId(ProgramIdValueNode),
    Resolver(ResolverValueNode),

    // ValueNodes.
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(ConstantValueNode),
    Enum(EnumValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    Number(NumberValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(SomeValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),

    // LinkNodes.
    ProgramLink(ProgramLinkNode),
}

impl From<ValueNode> for InstructionInputValueNode {
    fn from(value: ValueNode) -> Self {
        match value {
            ValueNode::Array(value) => Self::Array(value),
            ValueNode::Boolean(value) => Self::Boolean(value),
            ValueNode::Bytes(value) => Self::Bytes(value),
            ValueNode::Constant(value) => Self::Constant(value),
            ValueNode::Enum(value) => Self::Enum(value),
            ValueNode::Map(value) => Self::Map(value),
            ValueNode::None(value) => Self::None(value),
            ValueNode::Number(value) => Self::Number(value),
            ValueNode::PublicKey(value) => Self::PublicKey(value),
            ValueNode::Set(value) => Self::Set(value),
            ValueNode::Some(value) => Self::Some(value),
            ValueNode::String(value) => Self::String(value),
            ValueNode::Struct(value) => Self::Struct(value),
            ValueNode::Tuple(value) => Self::Tuple(value),
        }
    }
}

impl TryFrom<InstructionInputValueNode> for ValueNode {
    type Error = CodamaError;

    fn try_from(value: InstructionInputValueNode) -> Result<Self, Self::Error> {
        match value {
            InstructionInputValueNode::Array(value) => Ok(Self::Array(value)),
            InstructionInputValueNode::Boolean(value) => Ok(Self::Boolean(value)),
            InstructionInputValueNode::Bytes(value) => Ok(Self::Bytes(value)),
            InstructionInputValueNode::Constant(value) => Ok(Self::Constant(value)),
            InstructionInputValueNode::Enum(value) => Ok(Self::Enum(value)),
            InstructionInputValueNode::Map(value) => Ok(Self::Map(value)),
            InstructionInputValueNode::None(value) => Ok(Self::None(value)),
            InstructionInputValueNode::Number(value) => Ok(Self::Number(value)),
            InstructionInputValueNode::PublicKey(value) => Ok(Self::PublicKey(value)),
            InstructionInputValueNode::Set(value) => Ok(Self::Set(value)),
            InstructionInputValueNode::Some(value) => Ok(Self::Some(value)),
            InstructionInputValueNode::String(value) => Ok(Self::String(value)),
            InstructionInputValueNode::Struct(value) => Ok(Self::Struct(value)),
            InstructionInputValueNode::Tuple(value) => Ok(Self::Tuple(value)),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: value.kind().to_string(),
                into: "ValueNode".to_string(),
            }),
        }
    }
}
