use super::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    MapValueNode, NoneValueNode, NumberValueNode, SomeValueNode, StringValueNode, StructValueNode,
    TupleValueNode,
};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq)]
pub enum ValueNode {
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(Box<ConstantValueNode>),
    Enum(EnumValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    Number(NumberValueNode),
    Some(Box<SomeValueNode>),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}
