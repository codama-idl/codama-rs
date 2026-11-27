use crate::utils::{FromMeta, SetOnce};
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

/// Parses a `date_time(...)` or `duration(...)` directive, named `directive`, into the integer that
/// encodes its value and its `ticks_per_second`, if any, which the spec requires to be non-zero.
pub(crate) fn parse_time_type(
    meta: &Meta,
    directive: &str,
) -> syn::Result<(IntegerTypeNode, Option<u32>)> {
    let pl = meta.assert_directive(directive)?.as_path_list()?;
    let mut number: SetOnce<IntegerTypeNode> = SetOnce::new("number");
    let mut ticks_per_second: SetOnce<u32> = SetOnce::new("ticks_per_second");

    pl.each(|ref meta| {
        // Lists and paths go to the type parser first, so `number(u64)` gets its hint, not the `number =` field.
        if meta.is_path_or_list() {
            return number.set(parse_encoding_integer(meta)?, meta);
        }
        match meta.path_str().as_str() {
            "number" => number.set(parse_encoding_integer(meta.as_value()?)?, meta),
            "ticks_per_second" => {
                ticks_per_second.set(parse_non_zero_u32(meta, "ticks_per_second")?, meta)
            }
            _ => Err(meta.error("unrecognized attribute")),
        }
    })?;

    Ok((number.take(meta)?, ticks_per_second.option()))
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
