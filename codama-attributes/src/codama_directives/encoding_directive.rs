use crate::{Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::BytesEncoding;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct EncodingDirective {
    pub encoding: BytesEncoding,
}

impl EncodingDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let path = meta.assert_directive("encoding")?.as_value()?.as_path()?;
        match BytesEncoding::try_from(path.to_string()) {
            Ok(encoding) => Ok(Self { encoding }),
            _ => Err(path.error("invalid encoding")),
        }
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a EncodingDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive {
            CodamaDirective::Encoding(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "encoding".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a EncodingDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
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
