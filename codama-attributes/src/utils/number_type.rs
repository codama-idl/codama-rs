use crate::utils::SetOnce;
use codama_nodes::Endianness;
use codama_syn_helpers::{extensions::*, Meta};

/// The arguments shared by `integer(...)` and `float(...)`, where `F` is the format and `D` the display.
pub(crate) struct NumberTypeArguments<F, D> {
    pub format: F,
    pub endian: Endianness,
    pub unit: Option<String>,
    pub display: Option<D>,
}

/// Parses `directive(format, endian, unit = "...", display = ...)`, where the format and the endian can also be
/// positional, such as `integer(u64, be)`. The endian defaults to little-endian.
pub(crate) fn parse_number_type<F, D>(
    meta: &Meta,
    directive: &str,
    parse_display: impl Fn(&Meta) -> syn::Result<D>,
) -> syn::Result<NumberTypeArguments<F, D>>
where
    F: for<'s> TryFrom<&'s str>,
{
    let pl = meta.assert_directive(directive)?.as_path_list()?;
    let mut format = SetOnce::<F>::new("format");
    let mut endian = SetOnce::<Endianness>::new("endian").initial_value(Endianness::Le);
    let mut unit = SetOnce::<String>::new("unit");
    let mut display = SetOnce::<D>::new("display");

    pl.each(|ref meta| match meta.path_str().as_str() {
        "format" => {
            let path = meta.as_value()?.as_path()?;
            match F::try_from(path.to_string().as_str()) {
                Ok(value) => format.set(value, meta),
                Err(_) => Err(path.error("invalid format")),
            }
        }
        "endian" => {
            let path = meta.as_value()?.as_path()?;
            match Endianness::try_from(path.to_string().as_str()) {
                Ok(value) => endian.set(value, meta),
                Err(_) => Err(path.error("invalid endian")),
            }
        }
        "unit" => unit.set(meta.as_value()?.as_expr()?.as_string()?, meta),
        "display" => display.set(parse_display(meta.as_value()?)?, meta),
        _ => {
            if let Ok(path) = meta.as_path() {
                if let Ok(value) = F::try_from(path.to_string().as_str()) {
                    return format.set(value, meta);
                }
                if let Ok(value) = Endianness::try_from(path.to_string().as_str()) {
                    return endian.set(value, meta);
                }
            }
            Err(meta.path()?.error("unrecognized attribute"))
        }
    })?;

    Ok(NumberTypeArguments {
        format: format.take(meta)?,
        endian: endian.take(meta)?,
        unit: unit.option(),
        display: display.option(),
    })
}
