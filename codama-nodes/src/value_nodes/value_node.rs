use super::{ArrayValueNode, BooleanValueNode, BytesValueNode, NumberValueNode};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq)]
pub enum ValueNode {
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Number(NumberValueNode),
    Bytes(BytesValueNode),
}
