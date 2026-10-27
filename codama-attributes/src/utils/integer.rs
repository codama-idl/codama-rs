use crate::utils::FromMeta;
use codama_nodes::{HasTransforms, IntegerTypeNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

/// Builds a node around an integer and moves the integer transforms to that node, as `@codama/upgrade` does.
/// They keep their order and apply after the transforms the node already has.
pub fn hoist_integer_transforms<T: HasTransforms>(
    mut number: IntegerTypeNode,
    wrap: impl FnOnce(IntegerTypeNode) -> T,
) -> T {
    let transforms = std::mem::take(&mut number.transforms);
    let mut node = wrap(number);
    node.transforms_mut().extend(transforms);
    node
}

/// Parses the integer that encodes the value of a `date_time`, `duration` or `fixed_point` type.
/// The spec makes it a pure encoding slot, so it cannot have a unit or a display of its own.
pub(crate) fn parse_encoding_integer(meta: &Meta) -> syn::Result<IntegerTypeNode> {
    let integer = IntegerTypeNode::try_from_type_node(TypeNode::from_meta(meta)?)
        .map_err(|_| meta.error("number must be an IntegerTypeNode"))?;
    if integer.unit.is_some() || integer.display.is_some() {
        return Err(meta.error("number must not have a unit or a display"));
    }
    Ok(integer)
}

/// Parses the `ticks_per_second` value of a `date_time` or `duration` type, which the spec requires to be non-zero.
pub(crate) fn parse_ticks_per_second(meta: &Meta) -> syn::Result<u32> {
    let value = meta.as_value()?;
    match value.as_expr()?.as_unsigned_integer()? {
        0 => Err(value.error("ticks_per_second must be greater than zero")),
        ticks_per_second => Ok(ticks_per_second),
    }
}
