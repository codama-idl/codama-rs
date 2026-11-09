use crate::utils::{parse_number_type, parse_unit_display, FromMeta};
use codama_nodes::{FloatFormat, FloatTypeNode};
use codama_syn_helpers::Meta;

impl FromMeta for FloatTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let arguments = parse_number_type::<FloatFormat, _>(meta, "float", |meta| {
            parse_unit_display(meta, "floats")
        })?;
        Ok(FloatTypeNode {
            unit: arguments.unit,
            display: arguments.display,
            ..FloatTypeNode::new(arguments.format, arguments.endian)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        FloatFormat::{F32, F64},
        StringValueNode, UnitNumberDisplayNode,
    };

    #[test]
    fn parses() {
        for (meta, expected) in [
            (syn::parse_quote! { float(f32) }, FloatTypeNode::le(F32)),
            (syn::parse_quote! { float(f64, be) }, FloatTypeNode::be(F64)),
            (syn::parse_quote! { float(be, f64) }, FloatTypeNode::be(F64)),
            (
                syn::parse_quote! { float(format = f64, endian = be) },
                FloatTypeNode::be(F64),
            ),
            (
                syn::parse_quote! { float(f64, unit = "USD") },
                FloatTypeNode {
                    unit: Some("USD".to_string()),
                    ..FloatTypeNode::le(F64)
                },
            ),
            (
                syn::parse_quote! { float(unit = "%", format = f32, endian = be) },
                FloatTypeNode {
                    unit: Some("%".to_string()),
                    ..FloatTypeNode::be(F32)
                },
            ),
        ] {
            assert_eq!(FloatTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn unit_display() {
        let display = || Some(UnitNumberDisplayNode::new(StringValueNode::new("EUR")));
        for (meta, expected) in [
            (
                syn::parse_quote! { float(f64, display = unit("EUR")) },
                FloatTypeNode {
                    display: display(),
                    ..FloatTypeNode::le(F64)
                },
            ),
            (
                syn::parse_quote! { float(f64, be, unit = "USD", display = unit("EUR")) },
                FloatTypeNode {
                    unit: Some("USD".to_string()),
                    display: display(),
                    ..FloatTypeNode::be(F64)
                },
            ),
        ] {
            assert_eq!(FloatTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn amount_display() {
        for meta in [
            syn::parse_quote! { float(f64, display = amount(decimals = 2, unit = "USD")) },
            syn::parse_quote! { float(f64, display = amount(unit = "USD")) },
            syn::parse_quote! { float(f64, display = amount) },
        ] {
            let error = FloatTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(
                error.to_string(),
                "floats take a unit display, use unit(...)"
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (syn::parse_quote! { float(le) }, "format is missing"),
            (syn::parse_quote! { float(u64) }, "unrecognized attribute"),
            (
                syn::parse_quote! { float(f64, unit = "USD", unit = "EUR") },
                "unit is already set",
            ),
            (
                syn::parse_quote! { float(f64, unit = USD) },
                "expected a string",
            ),
            (
                syn::parse_quote! { float(f64, display = unit("USD"), display = unit("EUR")) },
                "display is already set",
            ),
            (
                syn::parse_quote! { float(f64, display = unit(USD)) },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { float(f64, display = date_time) },
                "date_time display is removed in v2, use the date_time(...) type",
            ),
            (
                syn::parse_quote! { float(f64, display = string) },
                "unrecognized number display",
            ),
        ] {
            let error = FloatTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
