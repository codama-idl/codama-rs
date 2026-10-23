use crate::{
    AccountBumpValueNode, AccountDataValueNode, AccountValueNode, ArrayValueNode, BooleanValueNode,
    BytesValueNode, ConditionalValueNode, ConstantValueNode, DataValueNode, EnumValueNode,
    FloatValueNode, IdentityValueNode, InjectedValueNode, IntegerValueNode, MapValueNode,
    NoneValueNode, PayerValueNode, PdaValueNode, ProgramIdValueNode, ProgramLinkNode,
    PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode, StructValueNode,
    TupleValueNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionInputValueNode {
    AccountBumpValue(AccountBumpValueNode),
    AccountDataValue(AccountDataValueNode),
    AccountValue(AccountValueNode),
    ArrayValue(ArrayValueNode),
    BooleanValue(BooleanValueNode),
    BytesValue(BytesValueNode),
    ConditionalValue(ConditionalValueNode),
    ConstantValue(ConstantValueNode),
    DataValue(DataValueNode),
    EnumValue(EnumValueNode),
    FloatValue(FloatValueNode),
    IdentityValue(IdentityValueNode),
    InjectedValue(InjectedValueNode),
    IntegerValue(IntegerValueNode),
    MapValue(MapValueNode),
    NoneValue(NoneValueNode),
    PayerValue(PayerValueNode),
    PdaValue(PdaValueNode),
    ProgramIdValue(ProgramIdValueNode),
    ProgramLink(ProgramLinkNode),
    PublicKeyValue(PublicKeyValueNode),
    SetValue(SetValueNode),
    SomeValue(SomeValueNode),
    StringValue(StringValueNode),
    StructValue(StructValueNode),
    TupleValue(TupleValueNode),
}
