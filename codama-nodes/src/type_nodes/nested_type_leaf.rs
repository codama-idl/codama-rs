use crate::{HasKind, TypeNode, TypeNodeTrait};
use codama_errors::CodamaResult;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NestedTypeLeaf(pub TypeNode);

impl TypeNodeTrait for NestedTypeLeaf {
    fn try_from_type_node(node: TypeNode) -> CodamaResult<Self> {
        Ok(Self(node))
    }
    fn into_type_node(self) -> TypeNode {
        self.0
    }
}

impl HasKind for NestedTypeLeaf {
    fn kind(&self) -> &'static str {
        self.0.kind()
    }
}
