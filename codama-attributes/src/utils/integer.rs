use crate::utils::FromMeta;
use codama_nodes::{IntegerTypeNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses the integer that encodes the value of a `date_time`, `duration` or `fixed_point` type.
/// The spec makes it a pure encoding slot, so it cannot have a unit or a display of its own.
pub(crate) fn parse_encoding_integer(meta: &Meta) -> syn::Result<IntegerTypeNode> {
    let integer = IntegerTypeNode::try_from_type_node(TypeNode::from_meta(meta)?)
        .map_err(|_| meta.error("number must be an integer(...) type"))?;
    if integer.unit.is_some() || integer.display.is_some() {
        return Err(meta.error("number must not have a unit or a display"));
    }
    Ok(integer)
}

/// Parses the `ticks_per_second` value of a `date_time` or `duration` type, which the spec requires to be non-zero.
pub(crate) fn parse_ticks_per_second(meta: &Meta) -> syn::Result<u32> {
    parse_non_zero_u32(meta, "ticks_per_second")
}

/// Parses the value of `meta`, named `name` in errors, as a `u32` that the spec requires to be non-zero.
pub(crate) fn parse_non_zero_u32(meta: &Meta, name: &str) -> syn::Result<u32> {
    let value = meta.as_value()?;
    // Parsing a wider integer first tells an out-of-range number apart from a value that is not a number.
    let number = value.as_expr()?.as_unsigned_integer::<u128>()?;
    match u32::try_from(number) {
        Ok(0) => Err(value.error(format!("{name} must be greater than zero"))),
        Ok(number) => Ok(number),
        Err(_) => Err(value.error(format!("{name} must be at most {}", u32::MAX))),
    }
}
