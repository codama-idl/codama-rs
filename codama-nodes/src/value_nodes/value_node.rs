use super::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    MapValueNode, NoneValueNode, NumberValueNode, PublicKeyValueNode, SetValueNode, SomeValueNode,
    StringValueNode, StructValueNode, TupleValueNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum ValueNode {
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(Box<ConstantValueNode>),
    Enum(EnumValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    Number(NumberValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(Box<SomeValueNode>),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}
