use super::{ArrayValueNode, NumberValueNode};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq)]
pub enum ValueNode {
    Array(ArrayValueNode),
    Number(NumberValueNode),
}
