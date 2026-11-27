use crate::utils::FromMeta;
use codama_nodes::{IntegerTypeNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `meta` as an `integer(...)` type, naming the slot it fills in errors, e.g. `prefix must be an
/// integer(...) type`. The error points at `meta`, so callers pass the type itself: `u16` in both
/// `option(u8, u16)` and `option(u8, prefix = u16)`.
pub(crate) fn parse_integer_type(meta: &Meta, name: &str) -> syn::Result<IntegerTypeNode> {
    IntegerTypeNode::try_from_type_node(TypeNode::from_meta(meta)?)
        .map_err(|_| meta.error(format!("{name} must be an integer(...) type")))
}

/// Parses the integer that encodes the value of a `date_time`, `duration` or `fixed_point` type.
/// The spec makes it a pure encoding slot, so it cannot have a unit or a display of its own.
pub(crate) fn parse_encoding_integer(meta: &Meta) -> syn::Result<IntegerTypeNode> {
    let integer = parse_integer_type(meta, "number")?;
    if integer.unit.is_some() || integer.display.is_some() {
        return Err(meta.error("number must not have a unit or a display"));
    }
    Ok(integer)
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
