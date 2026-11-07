use crate::utils::FromMeta;
use codama_nodes::{IntegerTypeNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

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
