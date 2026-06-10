use crate::{AccountValueNode, ArgumentValueNode, CamelCaseString, HasName};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaValueProgramId {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
}

impl HasName for PdaValueProgramId {
    fn name(&self) -> &CamelCaseString {
        match self {
            PdaValueProgramId::Account(node) => node.name(),
            PdaValueProgramId::Argument(node) => node.name(),
        }
    }
}
