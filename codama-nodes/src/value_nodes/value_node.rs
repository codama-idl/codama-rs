use super::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    NumberValueNode, StringValueNode, StructValueNode, TupleValueNode,
};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq)]
pub enum ValueNode {
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(Box<ConstantValueNode>),
    Enum(EnumValueNode),
    Number(NumberValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}
