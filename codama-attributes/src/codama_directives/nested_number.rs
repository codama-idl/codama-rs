use crate::utils::FromMeta;
use codama_nodes::{NestedTypeNode, NumberTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parse a type node that must resolve to a number, allowing the wrappers
/// `NestedTypeNode` accepts, e.g. `number(u32)` or `fixed_size(number(u32), 4)`.
pub(crate) fn nested_number(
    meta: &Meta,
    ident: &str,
) -> syn::Result<NestedTypeNode<NumberTypeNode>> {
    let node = TypeNode::from_meta(meta)?;
    NestedTypeNode::<NumberTypeNode>::try_from(node)
        .map_err(|_| meta.error(format!("{ident} must be a NumberTypeNode")))
}
