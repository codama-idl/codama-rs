use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{ConstantValueNode, TypeNode, ValueNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ConstantValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("constant")?.as_path_list()?;
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut value: SetOnce<ValueNode> = SetOnce::new("value");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "value" => value.set(ValueNode::from_meta(meta.as_value()?)?, meta),
            // Positional args fill `type` then `value`, since some names such as
            // `public_key` exist as both a type node and a value node. Anything
            // else named is a typo rather than a positional arg.
            _ => {
                if !meta.is_path_or_list() && meta.as_expr().is_err() {
                    return Err(meta.error("unrecognized attribute"));
                }
                match r#type.is_set() {
                    false => r#type.set(TypeNode::from_meta(meta)?, meta),
                    true => value.set(ValueNode::from_meta(meta)?, meta),
                }
            }
        })?;

        Ok(ConstantValueNode::new(
            r#type.take(meta)?,
            value.take(meta)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        NumberFormat::U8, NumberTypeNode, NumberValueNode, StringTypeNode, StringValueNode,
    };

    fn parse(meta: Meta) -> syn::Result<ConstantValueNode> {
        ConstantValueNode::from_meta(&meta)
    }

    #[test]
    fn implicit() {
        assert_eq!(
            parse(syn::parse_quote! { constant(number(u8), 42) }).unwrap(),
            ConstantValueNode::new(NumberTypeNode::le(U8), NumberValueNode::new(42u64))
        );
    }

    #[test]
    fn explicit() {
        assert_eq!(
            parse(syn::parse_quote! { constant(type = string, value = "hello") }).unwrap(),
            ConstantValueNode::new(StringTypeNode::utf8(), StringValueNode::new("hello"))
        );
    }

    #[test]
    fn value_missing() {
        let error = parse(syn::parse_quote! { constant(number(u8)) }).unwrap_err();
        assert_eq!(error.to_string(), "value is missing");
    }

    #[test]
    fn type_missing() {
        let error = parse(syn::parse_quote! { constant(value = 42) }).unwrap_err();
        assert_eq!(error.to_string(), "type is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        let error = parse(syn::parse_quote! { constant(foo = 42) }).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized attribute");
    }
}
