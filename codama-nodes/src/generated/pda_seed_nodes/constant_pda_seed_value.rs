use crate::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    FloatValueNode, InjectedValueNode, IntegerValueNode, MapValueNode, NoneValueNode,
    ProgramIdValueNode, PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode,
    StructValueNode, TupleValueNode,
};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
#[non_exhaustive]
pub enum ConstantPdaSeedValue {
    ArrayValue(ArrayValueNode),
    BooleanValue(BooleanValueNode),
    BytesValue(BytesValueNode),
    ConstantValue(ConstantValueNode),
    EnumValue(EnumValueNode),
    FloatValue(FloatValueNode),
    InjectedValue(InjectedValueNode),
    IntegerValue(IntegerValueNode),
    MapValue(MapValueNode),
    NoneValue(NoneValueNode),
    ProgramIdValue(ProgramIdValueNode),
    PublicKeyValue(PublicKeyValueNode),
    SetValue(SetValueNode),
    SomeValue(SomeValueNode),
    StringValue(StringValueNode),
    StructValue(StructValueNode),
    TupleValue(TupleValueNode),
}
