use super::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    MapValueNode, NumberValueNode, StringValueNode, StructValueNode, TupleValueNode,
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
    Number(NumberValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}
