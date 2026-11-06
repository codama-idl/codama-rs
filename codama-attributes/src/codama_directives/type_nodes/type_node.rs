use crate::{
    utils::{removed_syntax, FromMeta},
    TypeDirectiveNode,
};
use codama_nodes::{
    ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode, EnumTypeNode,
    EnumVariantTypeNode, FloatTypeNode, IntegerTypeNode, MapTypeNode, OptionTypeNode,
    PublicKeyTypeNode, RegisteredTypeNode, RemainderOptionTypeNode, SetTypeNode, StringTypeNode,
    StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNode, ZeroableOptionTypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RegisteredTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "array" => ArrayTypeNode::from_meta(meta).map(Self::from),
            "boolean" => BooleanTypeNode::from_meta(meta).map(Self::from),
            "bytes" => BytesTypeNode::from_meta(meta).map(Self::from),
            "date_time" => DateTimeTypeNode::from_meta(meta).map(Self::from),
            "enum" => EnumTypeNode::from_meta(meta).map(Self::from),
            "field" => StructFieldTypeNode::from_meta(meta).map(Self::from),
            "float" => FloatTypeNode::from_meta(meta).map(Self::from),
            "integer" => IntegerTypeNode::from_meta(meta).map(Self::from),
            "map" => MapTypeNode::from_meta(meta).map(Self::from),
            "number" => removed_syntax(meta, "number(...)", "integer(...) or float(...)"),
            "option" => OptionTypeNode::from_meta(meta).map(Self::from),
            "public_key" => PublicKeyTypeNode::from_meta(meta).map(Self::from),
            "remainder_option" => RemainderOptionTypeNode::from_meta(meta).map(Self::from),
            "set" => SetTypeNode::from_meta(meta).map(Self::from),
            "string" => StringTypeNode::from_meta(meta).map(Self::from),
            "struct" => StructTypeNode::from_meta(meta).map(Self::from),
            "tuple" => TupleTypeNode::from_meta(meta).map(Self::from),
            "variant" => EnumVariantTypeNode::from_meta(meta).map(Self::from),
            "zeroable_option" => ZeroableOptionTypeNode::from_meta(meta).map(Self::from),
            _ => Err(meta.error("unrecognized type")),
        }
    }
}

impl FromMeta for TypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Self::try_from(TypeDirectiveNode::from_meta(meta)?)
            .map_err(|_| meta.error("unrecognized type"))
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_type_err;

    #[test]
    fn number_is_removed() {
        assert_type_err!(
            { number(u64) },
            "number(...) is removed in v2, use integer(...) or float(...)"
        );
        assert_type_err!(
            { number(f64, be) },
            "number(...) is removed in v2, use integer(...) or float(...)"
        );
    }
}
