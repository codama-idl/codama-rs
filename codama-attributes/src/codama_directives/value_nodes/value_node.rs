use crate::codama_directives::value_nodes::{
    number_value_node::signed_literal, program::public_key_value_node_from_program,
    sysvar::public_key_value_node_from_sysvar,
};
use crate::utils::FromMeta;
use codama_nodes::{
    BooleanValueNode, FloatValueNode, IntegerValueNode, PublicKeyValueNode, StringValueNode,
    ValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};
use syn::Lit;

impl FromMeta for ValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "public_key" => PublicKeyValueNode::from_meta(meta).map(Self::from),
            "program" => public_key_value_node_from_program(meta).map(Self::from),
            "sysvar" => public_key_value_node_from_sysvar(meta).map(Self::from),
            // The kind of the literal picks the value node, so its own error, such as a number
            // out of range, reaches the user.
            _ => match meta.as_expr().ok().and_then(signed_literal) {
                Some(("", Lit::Bool(_))) => BooleanValueNode::from_meta(meta).map(Self::from),
                Some(("", Lit::Str(_))) => StringValueNode::from_meta(meta).map(Self::from),
                Some((_, Lit::Int(_))) => IntegerValueNode::from_meta(meta).map(Self::from),
                Some((_, Lit::Float(_))) => FloatValueNode::from_meta(meta).map(Self::from),
                _ => Err(meta.error("unrecognized value")),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_range_numbers_keep_their_error() {
        for (meta, expected) in [
            (
                syn::parse_quote! { 340282366920938463463374607431768211456 },
                "number too large to fit in target type",
            ),
            (
                syn::parse_quote! { 1e400 },
                "number out of the range of an f64",
            ),
            (
                syn::parse_quote! { -1e-400 },
                "number out of the range of an f64",
            ),
        ] {
            let error = ValueNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn unrecognized_values() {
        for meta in [
            syn::parse_quote! { foo },
            syn::parse_quote! { -true },
            syn::parse_quote! { -"a" },
            syn::parse_quote! { 'c' },
        ] {
            let error = ValueNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), "unrecognized value");
        }
    }
}
