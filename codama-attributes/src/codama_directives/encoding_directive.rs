use codama_nodes::BytesEncoding;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct EncodingDirective {
    pub encoding: BytesEncoding,
}

impl EncodingDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("encoding")?.as_path_value()?;
        let value = pv.value.as_path()?;
        match BytesEncoding::try_from(value.to_string()) {
            Ok(encoding) => Ok(Self { encoding }),
            _ => Err(value.error("invalid encoding")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { encoding = base64 };
        let directive = EncodingDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EncodingDirective {
                encoding: BytesEncoding::Base64
            }
        );
    }
}
