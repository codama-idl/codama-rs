use crate::{utils::SetOnce, NodeAttributeParse};
use codama_nodes::{Endian, Node, NumberFormat, NumberTypeNode};
use codama_syn_helpers::{syn_traits::*, Meta};

impl NodeAttributeParse for NumberTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Node> {
        let mut format = SetOnce::<NumberFormat>::new("format");
        let mut endian = SetOnce::<Endian>::new("endian").initial_value(Endian::Little);
        meta.as_list()?.parse_metas(|ref meta| {
            let path = meta.path()?;
            match (meta.path_str().as_str(), meta) {
                ("format", Meta::NameValue(m)) => match m.value.as_path()?.to_string().as_str() {
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
                    _ => Err(m.value.error("invalid format")),
                },
                ("endian", Meta::NameValue(m)) => match m.value.as_path()?.to_string().as_str() {
                    "le" => endian.set(Endian::Little, meta),
                    "be" => endian.set(Endian::Big, meta),
                    _ => Err(m.value.error("invalid endian")),
                },
                ("u8", Meta::Path(_)) => format.set(NumberFormat::U8, meta),
                ("u16", Meta::Path(_)) => format.set(NumberFormat::U16, meta),
                ("u32", Meta::Path(_)) => format.set(NumberFormat::U32, meta),
                ("u64", Meta::Path(_)) => format.set(NumberFormat::U64, meta),
                ("u128", Meta::Path(_)) => format.set(NumberFormat::U128, meta),
                ("i8", Meta::Path(_)) => format.set(NumberFormat::I8, meta),
                ("i16", Meta::Path(_)) => format.set(NumberFormat::I16, meta),
                ("i32", Meta::Path(_)) => format.set(NumberFormat::I32, meta),
                ("i64", Meta::Path(_)) => format.set(NumberFormat::I64, meta),
                ("i128", Meta::Path(_)) => format.set(NumberFormat::I128, meta),
                ("f32", Meta::Path(_)) => format.set(NumberFormat::F32, meta),
                ("f64", Meta::Path(_)) => format.set(NumberFormat::F64, meta),
                ("shortU16", Meta::Path(_)) => format.set(NumberFormat::ShortU16, meta),
                ("le", Meta::Path(_)) => endian.set(Endian::Little, meta),
                ("be", Meta::Path(_)) => endian.set(Endian::Big, meta),
                _ => Err(path.error("unrecognized attribute")),
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
    fn implicit() {
        assert_node!(#[node(number_type(u16, le))], NumberTypeNode::le(U16).into());
        assert_node!(#[node(number_type(u16, le))], NumberTypeNode::le(U16).into());
        assert_node!(#[node(number_type(u64, le))], NumberTypeNode::le(U64).into());
        assert_node!(#[node(number_type(u16, be))], NumberTypeNode::be(U16).into());
        assert_node!(#[node(number_type(u64, be))], NumberTypeNode::be(U64).into());
        assert_node!(#[node(number_type(le, u16))], NumberTypeNode::le(U16).into());
    }

    #[test]
    fn explicit() {
        assert_node!(#[node(number_type(format = u16, endian = le))], NumberTypeNode::le(U16).into());
        assert_node!(#[node(number_type(format = u16, endian = le))], NumberTypeNode::le(U16).into());
        assert_node!(#[node(number_type(format = u64, endian = le))], NumberTypeNode::le(U64).into());
        assert_node!(#[node(number_type(format = u16, endian = be))], NumberTypeNode::be(U16).into());
        assert_node!(#[node(number_type(format = u64, endian = be))], NumberTypeNode::be(U64).into());
        assert_node!(#[node(number_type(endian = le, format = u16))], NumberTypeNode::le(U16).into());
    }

    #[test]
    fn defaults_to_little_endian() {
        assert_node!(#[node(number_type(u16))], NumberTypeNode::le(U16).into());
        assert_node!(#[node(number_type(format = u16))], NumberTypeNode::le(U16).into());
    }

    #[test]
    fn missing_format() {
        assert_node_err!(#[node(number_type(le))], "format is missing");
    }

    #[test]
    fn format_already_set() {
        assert_node_err!(#[node(number_type(u8, u16))], "format is already set");
    }

    #[test]
    fn endian_already_set() {
        assert_node_err!(#[node(number_type(le, be))], "endian is already set");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_node_err!(#[node(number_type(u16, le, unknown))], "unrecognized attribute");
        assert_node_err!(#[node(number_type(u16, le, unknown = 42))], "unrecognized attribute");
        assert_node_err!(#[node(number_type(u16 = what?, le))], "unrecognized attribute");
    }

    #[test]
    fn expected_a_path() {
        assert_node_err!(#[node(number_type(42))], "expected a path");
    }
}
