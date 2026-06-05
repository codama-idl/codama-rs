use crate::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ArrayValueNode, BooleanValueNode,
    BytesValueNode, ConditionalValueNode, ConstantValueNode, EnumValueNode, IdentityValueNode,
    MapValueNode, NoneValueNode, NumberValueNode, PayerValueNode, PdaValueNode, ProgramIdValueNode,
    ProgramLinkNode, PublicKeyValueNode, ResolverValueNode, SetValueNode, SomeValueNode,
    StringValueNode, StructValueNode, TupleValueNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionInputValueNode {
    AccountBumpValue(AccountBumpValueNode),
    AccountValue(AccountValueNode),
    ArgumentValue(ArgumentValueNode),
    ArrayValue(ArrayValueNode),
    BooleanValue(BooleanValueNode),
    BytesValue(BytesValueNode),
    ConditionalValue(ConditionalValueNode),
    ConstantValue(ConstantValueNode),
    EnumValue(EnumValueNode),
    IdentityValue(IdentityValueNode),
    MapValue(MapValueNode),
    NoneValue(NoneValueNode),
    NumberValue(NumberValueNode),
    PayerValue(PayerValueNode),
    PdaValue(PdaValueNode),
    ProgramIdValue(ProgramIdValueNode),
    ProgramLink(ProgramLinkNode),
    PublicKeyValue(PublicKeyValueNode),
    ResolverValue(ResolverValueNode),
    SetValue(SetValueNode),
    SomeValue(SomeValueNode),
    StringValue(StringValueNode),
    StructValue(StructValueNode),
    TupleValue(TupleValueNode),
}
