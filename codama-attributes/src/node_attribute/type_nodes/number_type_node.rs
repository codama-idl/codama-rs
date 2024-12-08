use crate::{utils::SetOnce, NodeAttributeParse};
use codama_errors::CodamaResult;
use codama_nodes::{Endian, Node, NumberFormat, NumberTypeNode};
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for NumberTypeNode {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node> {
        let mut format = SetOnce::<NumberFormat>::new("format", meta);
        let mut endian = SetOnce::<Endian>::new("endian", meta);
        meta.parse_nested_meta(|meta| match meta.path.last_str().as_str() {
            "u8" => format.set(NumberFormat::U8),
            "u16" => format.set(NumberFormat::U16),
            "u32" => format.set(NumberFormat::U32),
            "u64" => format.set(NumberFormat::U64),
            "u128" => format.set(NumberFormat::U128),
            "i8" => format.set(NumberFormat::I8),
            "i16" => format.set(NumberFormat::I16),
            "i32" => format.set(NumberFormat::I32),
            "i64" => format.set(NumberFormat::I64),
            "i128" => format.set(NumberFormat::I128),
            "f32" => format.set(NumberFormat::F32),
            "f64" => format.set(NumberFormat::F64),
            "shortU16" => format.set(NumberFormat::ShortU16),
            "le" => endian.set(Endian::Little),
            "be" => endian.set(Endian::Big),
            _ => Err(meta.error("numberTypeNode: unrecognized attribute")),
        })?;
        Ok(NumberTypeNode::new(format.take()?, endian.take()?).into())
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
}
