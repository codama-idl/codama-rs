use crate::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    MapValueNode, NoneValueNode, NumberValueNode, ProgramIdValueNode, PublicKeyValueNode,
    SetValueNode, SomeValueNode, StringValueNode, StructValueNode, TupleValueNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum ConstantPdaSeedValue {
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(ConstantValueNode),
    Enum(EnumValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    Number(NumberValueNode),
    ProgramId(ProgramIdValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(SomeValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}
