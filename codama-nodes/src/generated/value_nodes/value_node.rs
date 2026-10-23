use crate::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    FloatValueNode, HasKind, InjectedValueNode, IntegerValueNode, MapEntryValueNode, MapValueNode,
    NoneValueNode, PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode,
    StructFieldValueNode, StructValueNode, TupleValueNode,
};
use codama_nodes_derive::{node_union, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredValueNode {
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(ConstantValueNode),
    Enum(EnumValueNode),
    Float(FloatValueNode),
    Injected(InjectedValueNode),
    Integer(IntegerValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(SomeValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),

    #[registered]
    MapEntry(MapEntryValueNode),
    #[registered]
    StructField(StructFieldValueNode),
}
