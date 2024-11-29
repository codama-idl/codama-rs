use crate::{NodeTrait, NodeUnionTrait};
use std::fmt::Debug;

pub trait TypeNodeTrait: NodeTrait {}

pub trait TypeNodeUnionTrait: NodeUnionTrait {}

pub trait NestedTypeNodeTrait<T: TypeNodeTrait>:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    fn get_nested_type_node(&self) -> &T;
}
