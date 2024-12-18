use crate::{NodeTrait, NodeUnionTrait, TypeNode};
use codama_errors::CodamaResult;
use std::fmt::Debug;

pub trait TypeNodeTrait: NodeTrait {
    fn from_type_node(_node: TypeNode) -> CodamaResult<Self> {
        Err(codama_errors::CodamaError::InvalidNodeConversion {
            from: "TypeNode".into(),
            into: Self::KIND.into(),
        })
    }
}

pub trait TypeNodeUnionTrait: NodeUnionTrait {}

pub trait NestedTypeNodeTrait<T: TypeNodeTrait>:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    type Mapped<U: TypeNodeTrait>: NestedTypeNodeTrait<U>;
    fn get_nested_type_node(&self) -> &T;
    fn map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> U>(self, f: F) -> Self::Mapped<U>;
}
