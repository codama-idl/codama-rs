use crate::{
    AccountValueNode, ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode,
    DataValueNode, EnumValueNode, FloatValueNode, InjectedValueNode, IntegerValueNode,
    MapValueNode, NoneValueNode, PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode,
    StructValueNode, TupleValueNode,
};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
#[non_exhaustive]
pub enum PdaSeedValueValue {
    AccountValue(AccountValueNode),
    ArrayValue(ArrayValueNode),
    BooleanValue(BooleanValueNode),
    BytesValue(BytesValueNode),
    ConstantValue(ConstantValueNode),
    DataValue(DataValueNode),
    EnumValue(EnumValueNode),
    FloatValue(FloatValueNode),
    InjectedValue(InjectedValueNode),
    IntegerValue(IntegerValueNode),
    MapValue(MapValueNode),
    NoneValue(NoneValueNode),
    PublicKeyValue(PublicKeyValueNode),
    SetValue(SetValueNode),
    SomeValue(SomeValueNode),
    StringValue(StringValueNode),
    StructValue(StructValueNode),
    TupleValue(TupleValueNode),
}
