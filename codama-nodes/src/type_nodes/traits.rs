use crate::{HasKind, TypeNode};
use codama_errors::CodamaResult;
use std::fmt::Debug;

pub trait TypeNodeTrait:
    HasKind + Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    fn try_from_type_node(node: TypeNode) -> CodamaResult<Self>;
    fn into_type_node(self) -> TypeNode;
}
