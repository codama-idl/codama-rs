use crate::{
    utils::{FromMeta, SetOnce},
    Attribute, Attributes, CodamaAttribute, CodamaDirective, TryFromFilter,
};
use codama_errors::CodamaError;
use codama_nodes::{
    BytesEncoding, CamelCaseString, ConstantDiscriminatorNode, ConstantValueNode,
    DiscriminatorNode, FieldDiscriminatorNode, SizeDiscriminatorNode,
};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct DiscriminatorDirective {
    pub discriminator: DiscriminatorNode,
}

impl DiscriminatorDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("discriminator")?.as_path_list()?;

        let kind = pl
            .parse_metas()?
            .iter()
            .find_map(|m| match m.path_str().as_str() {
                "bytes" => Some(DiscriminatorKind::Constant),
                "field" => Some(DiscriminatorKind::Field),
                "size" => Some(DiscriminatorKind::Size),
                _ => None,
            })
            .ok_or_else(|| meta.error("discriminator must specify one of: bytes, field, size"))?;

        let mut encoding_is_set: bool = false;
        let mut bytes_is_array: bool = false;
        let mut bytes = SetOnce::<BytesValue>::new("bytes");
        let mut encoding =
            SetOnce::<BytesEncoding>::new("encoding").initial_value(BytesEncoding::Base16);
        let mut field = SetOnce::<CamelCaseString>::new("field");
        let mut offset = SetOnce::<usize>::new("offset").initial_value(0);
        let mut size = SetOnce::<usize>::new("size");
        pl.each(|ref meta| match meta.path_str().as_str() {
            "bytes" => {
                if kind != DiscriminatorKind::Constant {
                    return Err(meta.error(format!("bytes cannot be used when {kind} is set")));
                }
                let value = BytesValue::from_meta(meta)?;
                if let BytesValue::Array(_) = value {
                    bytes_is_array = true;
                    if encoding_is_set {
                        return Err(meta.error("bytes must be a string when encoding is set"));
                    }
                };
                bytes.set(value, meta)
            }
            "encoding" => {
                if kind != DiscriminatorKind::Constant {
                    return Err(meta.error(format!("encoding cannot be used when {kind} is set")));
                }
                let value = BytesEncoding::from_meta(meta)?;
                encoding_is_set = true;
                if bytes_is_array {
                    return Err(meta.error("encoding cannot be set when bytes is an array"));
                }
                encoding.set(value, meta)
            }
            "field" => {
                if kind != DiscriminatorKind::Field {
                    return Err(meta.error(format!("field cannot be used when {kind} is set")));
                }
                field.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta)
            }
            "offset" => {
                if kind == DiscriminatorKind::Size {
                    return Err(meta.error(format!("offset cannot be used when {kind} is set")));
                }
                offset.set(meta.as_value()?.as_expr()?.as_unsigned_integer()?, meta)
            }
            "size" => {
                if kind != DiscriminatorKind::Size {
                    return Err(meta.error(format!("size cannot be used when {kind} is set")));
                }
                size.set(meta.as_value()?.as_expr()?.as_unsigned_integer()?, meta)
            }
            _ => Err(meta.error("unrecognized attribute")),
        })?;

        Ok(DiscriminatorDirective {
            discriminator: match kind {
                DiscriminatorKind::Constant => ConstantDiscriminatorNode::new(
                    ConstantValueNode::bytes(encoding.take(meta)?, bytes.take(meta)?),
                    offset.take(meta)?,
                )
                .into(),
                DiscriminatorKind::Field => {
                    FieldDiscriminatorNode::new(field.take(meta)?, offset.take(meta)?).into()
                }
                DiscriminatorKind::Size => SizeDiscriminatorNode::new(size.take(meta)?).into(),
            },
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a DiscriminatorDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Discriminator(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "discriminator".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a DiscriminatorDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

impl From<&DiscriminatorDirective> for DiscriminatorNode {
    fn from(directive: &DiscriminatorDirective) -> Self {
        directive.discriminator.clone()
    }
}

impl DiscriminatorDirective {
    pub fn nodes(attributes: &Attributes) -> Vec<DiscriminatorNode> {
        attributes
            .iter()
            .filter_map(DiscriminatorDirective::filter)
            .map(Into::into)
            .collect()
    }
}

#[derive(PartialEq, Debug)]
enum DiscriminatorKind {
    Constant,
    Field,
    Size,
}

impl std::fmt::Display for DiscriminatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscriminatorKind::Constant => write!(f, "bytes"),
            DiscriminatorKind::Field => write!(f, "field"),
            DiscriminatorKind::Size => write!(f, "size"),
        }
    }
}

enum BytesValue {
    Array(Vec<u8>),
    Encoded(String),
}

impl FromMeta for BytesValue {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let expr = match meta {
            Meta::Expr(expr) => Ok(expr.clone()),
            Meta::PathList(pl) => Ok(pl.as_expr_array()?.into()),
            _ => meta.as_value()?.as_expr().cloned(),
        }?;

        if let Ok(s) = expr.as_string() {
            return Ok(BytesValue::Encoded(s));
        }
        if let Ok(arr) = expr.as_u8_array() {
            return Ok(BytesValue::Array(arr));
        }
        Err(expr.error("expected a string or a byte array"))
    }
}

impl From<BytesValue> for String {
    fn from(value: BytesValue) -> Self {
        match value {
            BytesValue::Array(bytes) => {
                let mut s = String::with_capacity(bytes.len() * 2);
                for byte in bytes {
                    s.push_str(&format!("{:02x}", byte));
                }
                s
            }
            BytesValue::Encoded(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_discriminator() {
        let meta: Meta = syn::parse_quote! { discriminator(bytes = "01020304") };
        let directive = DiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            DiscriminatorDirective {
                discriminator: ConstantDiscriminatorNode::new(
                    ConstantValueNode::bytes(BytesEncoding::Base16, "01020304"),
                    0
                )
                .into(),
            }
        );
    }

    #[test]
    fn constant_discriminator_with_byte_array() {
        let meta: Meta = syn::parse_quote! { discriminator(bytes = [1, 2, 3, 4]) };
        let directive = DiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            DiscriminatorDirective {
                discriminator: ConstantDiscriminatorNode::new(
                    ConstantValueNode::bytes(BytesEncoding::Base16, "01020304"),
                    0
                )
                .into(),
            }
        );
    }

    #[test]
    fn constant_discriminator_with_encoding() {
        let meta: Meta = syn::parse_quote! { discriminator(bytes = "hello", encoding = "utf8") };
        let directive = DiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            DiscriminatorDirective {
                discriminator: ConstantDiscriminatorNode::new(
                    ConstantValueNode::bytes(BytesEncoding::Utf8, "hello"),
                    0
                )
                .into(),
            }
        );
    }

    #[test]
    fn constant_discriminator_with_offset() {
        let meta: Meta = syn::parse_quote! { discriminator(bytes = "ffff", offset = 42) };
        let directive = DiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            DiscriminatorDirective {
                discriminator: ConstantDiscriminatorNode::new(
                    ConstantValueNode::bytes(BytesEncoding::Base16, "ffff"),
                    42
                )
                .into(),
            }
        );
    }

    #[test]
    fn constant_discriminator_with_byte_array_and_encoding() {
        let meta: Meta =
            syn::parse_quote! { discriminator(bytes = [1, 2, 3, 4], encoding = "utf8") };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "encoding cannot be set when bytes is an array"
        );
    }

    #[test]
    fn constant_discriminator_with_too_many_bytes() {
        let meta: Meta =
            syn::parse_quote! { discriminator(bytes = [1, 2, 3, 4], bytes = "01020304") };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "bytes is already set");
    }

    #[test]
    fn constant_discriminator_with_too_many_encoding() {
        let meta: Meta = syn::parse_quote! { discriminator(bytes = "01020304", encoding = "utf8", encoding = "base64") };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "encoding is already set");
    }

    #[test]
    fn constant_discriminator_with_too_many_offsets() {
        let meta: Meta =
            syn::parse_quote! { discriminator(bytes = "01020304", offset = 42, offset = 43) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "offset is already set");
    }

    #[test]
    fn constant_discriminator_with_encoding_and_byte_array() {
        let meta: Meta =
            syn::parse_quote! { discriminator(encoding = "utf8", bytes = [1, 2, 3, 4]) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "bytes must be a string when encoding is set"
        );
    }

    #[test]
    fn constant_discriminator_with_another_discriminator_kind() {
        let meta: Meta =
            syn::parse_quote! { discriminator(bytes = "01020304", field = "account_type") };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "field cannot be used when bytes is set");
    }

    #[test]
    fn field_discriminator() {
        let meta: Meta = syn::parse_quote! { discriminator(field = "account_type") };
        let directive = DiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            DiscriminatorDirective {
                discriminator: FieldDiscriminatorNode::new("AccountType", 0).into(),
            }
        );
    }

    #[test]
    fn field_discriminator_with_offset() {
        let meta: Meta = syn::parse_quote! { discriminator(field = "account_type", offset = 42) };
        let directive = DiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            DiscriminatorDirective {
                discriminator: FieldDiscriminatorNode::new("AccountType", 42).into(),
            }
        );
    }

    #[test]
    fn field_discriminator_with_too_many_field_names() {
        let meta: Meta =
            syn::parse_quote! { discriminator(field = "account_type", field = "user_type") };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "field is already set");
    }

    #[test]
    fn field_discriminator_with_too_many_offsets() {
        let meta: Meta =
            syn::parse_quote! { discriminator(field = "account_type", offset = 42, offset = 43) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "offset is already set");
    }

    #[test]
    fn field_discriminator_with_another_discriminator_kind() {
        let meta: Meta = syn::parse_quote! { discriminator(field = "account_type", size = 100) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "size cannot be used when field is set");
    }

    #[test]
    fn size_discriminator() {
        let meta: Meta = syn::parse_quote! { discriminator(size = 100) };
        let directive = DiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            DiscriminatorDirective {
                discriminator: SizeDiscriminatorNode::new(100).into(),
            }
        );
    }

    #[test]
    fn size_discriminator_with_too_many_sizes() {
        let meta: Meta = syn::parse_quote! { discriminator(size = 100, size = 200) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "size is already set");
    }

    #[test]
    fn size_discriminator_with_offset() {
        let meta: Meta = syn::parse_quote! { discriminator(size = 100, offset = 42) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "offset cannot be used when size is set");
    }

    #[test]
    fn size_discriminator_with_another_discriminator_kind() {
        let meta: Meta = syn::parse_quote! { discriminator(size = 100, bytes = [1, 2, 3]) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "bytes cannot be used when size is set");
    }

    #[test]
    fn empty_discriminator() {
        let meta: Meta = syn::parse_quote! { discriminator() };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "discriminator must specify one of: bytes, field, size"
        );
    }

    #[test]
    fn discriminator_with_no_kind() {
        let meta: Meta = syn::parse_quote! { discriminator(encoding = "utf8", offset = 42) };
        let error = DiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "discriminator must specify one of: bytes, field, size"
        );
    }
}
