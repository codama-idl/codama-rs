use crate::{utils::SetOnce, NodeAttributeParse};
use codama_nodes::{Endian, Node, NumberFormat, NumberTypeNode};
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for NumberTypeNode {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Node> {
        let mut format = SetOnce::<NumberFormat>::new("format");
        let mut endian = SetOnce::<Endian>::new("endian");
        meta.parse_nested_meta(|ref meta| {
            meta.input.consume_arg()?;
            match meta.path.last_str().as_str() {
                "u8" => format.set(NumberFormat::U8, meta),
                "u16" => format.set(NumberFormat::U16, meta),
                "u32" => format.set(NumberFormat::U32, meta),
                "u64" => format.set(NumberFormat::U64, meta),
                "u128" => format.set(NumberFormat::U128, meta),
                "i8" => format.set(NumberFormat::I8, meta),
                "i16" => format.set(NumberFormat::I16, meta),
                "i32" => format.set(NumberFormat::I32, meta),
                "i64" => format.set(NumberFormat::I64, meta),
                "i128" => format.set(NumberFormat::I128, meta),
                "f32" => format.set(NumberFormat::F32, meta),
                "f64" => format.set(NumberFormat::F64, meta),
                "shortU16" => format.set(NumberFormat::ShortU16, meta),
                "le" => endian.set(Endian::Little, meta),
                "be" => endian.set(Endian::Big, meta),
                _ => Err(meta.error("unrecognized attribute")),
            }
        })?;
        Ok(NumberTypeNode::new(format.take(meta)?, endian.take(meta)?).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err, NodeAttribute};
    use codama_syn_helpers::syn_build;
    use quote::quote;
    use NumberFormat::{U16, U64};

    #[test]
    fn ok() {
        assert_node!(#[node(numberTypeNode(u16, le))], NumberTypeNode::le(U16).into());
        assert_node!(#[node(numberTypeNode(u64, le))], NumberTypeNode::le(U64).into());
        assert_node!(#[node(numberTypeNode(u16, be))], NumberTypeNode::be(U16).into());
        assert_node!(#[node(numberTypeNode(u64, be))], NumberTypeNode::be(U64).into());
        assert_node!(#[node(numberTypeNode(le, u16))], NumberTypeNode::le(U16).into());
    }

    #[test]
    fn missing_format() {
        assert_node_err!(#[node(numberTypeNode(le))], "format is missing");
    }

    #[test]
    fn format_already_set() {
        assert_node_err!(#[node(numberTypeNode(u8, u16))], "format is already set");
    }

    #[test]
    fn missing_endian() {
        assert_node_err!(#[node(numberTypeNode(u16))], "endian is missing");
    }

    #[test]
    fn endian_already_set() {
        assert_node_err!(#[node(numberTypeNode(le, be))], "endian is already set");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_node_err!(#[node(numberTypeNode(u16, le, unknown))], "unrecognized attribute");
        assert_node_err!(#[node(numberTypeNode(u16, le, unknown = 42))], "unrecognized attribute");
    }
}
