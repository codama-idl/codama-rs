use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{Endianness, FloatFormat, FloatTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for FloatTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("float")?.as_path_list()?;
        let mut format = SetOnce::<FloatFormat>::new("format");
        let mut endian = SetOnce::<Endianness>::new("endian").initial_value(Endianness::Le);

        pl.each(|ref meta| match meta.path_str().as_str() {
            "format" => {
                let path = meta.as_value()?.as_path()?;
                match FloatFormat::try_from(path.to_string().as_str()) {
                    Ok(value) => format.set(value, meta),
                    Err(_) => Err(path.error("invalid format")),
                }
            }
            "endian" => {
                let path = meta.as_value()?.as_path()?;
                match Endianness::try_from(path.to_string()) {
                    Ok(value) => endian.set(value, meta),
                    _ => Err(path.error("invalid endian")),
                }
            }
            _ => {
                if let Ok(path) = meta.as_path() {
                    if let Ok(value) = FloatFormat::try_from(path.to_string().as_str()) {
                        return format.set(value, meta);
                    }
                    if let Ok(value) = Endianness::try_from(path.to_string()) {
                        return endian.set(value, meta);
                    }
                }
                Err(meta.path()?.error("unrecognized attribute"))
            }
        })?;

        Ok(FloatTypeNode::new(format.take(meta)?, endian.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::FloatFormat::{F32, F64};

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
        ] {
            assert_eq!(FloatTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (syn::parse_quote! { float(le) }, "format is missing"),
            (syn::parse_quote! { float(u64) }, "unrecognized attribute"),
        ] {
            let error = FloatTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
