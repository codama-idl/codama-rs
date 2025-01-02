use crate::{HasKind, NodeUnionTrait, TypeNode};
use codama_errors::CodamaResult;
use std::fmt::Debug;

pub trait TypeNodeTrait:
    HasKind + Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    fn try_from_type_node(node: TypeNode) -> CodamaResult<Self>;
    fn into_type_node(self) -> TypeNode;
}

pub trait TypeNodeUnionTrait: NodeUnionTrait {}

pub trait NestedTypeNodeTrait<T: TypeNodeTrait>:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    type Mapped<U: TypeNodeTrait>: NestedTypeNodeTrait<U>;
    fn get_nested_type_node(&self) -> &T;
    fn try_map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> CodamaResult<U>>(
        self,
        f: F,
    ) -> CodamaResult<Self::Mapped<U>>;
    fn map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> U>(self, f: F) -> Self::Mapped<U> {
        self.try_map_nested_type_node(|value| Ok(f(value))).unwrap()
    }
}
