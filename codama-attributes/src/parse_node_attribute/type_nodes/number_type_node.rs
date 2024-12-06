use codama_errors::CodamaResult;
use codama_nodes::{Endian, Node, NumberFormat, NumberTypeNode};
use codama_syn_helpers::syn_traits::*;

pub fn number_type_node(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node> {
    let mut format: Option<NumberFormat> = None;
    let mut endian: Option<Endian> = None;
    meta.parse_nested_meta(|meta| {
        match meta.path.last_str().as_str() {
            "u8" => format = Some(NumberFormat::U8),
            "u16" => format = Some(NumberFormat::U16),
            "u32" => format = Some(NumberFormat::U32),
            "u64" => format = Some(NumberFormat::U64),
            "u128" => format = Some(NumberFormat::U128),
            "i8" => format = Some(NumberFormat::I8),
            "i16" => format = Some(NumberFormat::I16),
            "i32" => format = Some(NumberFormat::I32),
            "i64" => format = Some(NumberFormat::I64),
            "i128" => format = Some(NumberFormat::I128),
            "f32" => format = Some(NumberFormat::F32),
            "f64" => format = Some(NumberFormat::F64),
            "shortU16" => format = Some(NumberFormat::ShortU16),
            "le" => endian = Some(Endian::Little),
            "be" => endian = Some(Endian::Big),
            _ => return Err(meta.error("numberTypeNode: unrecognized attribute")),
        };
        Ok(())
    })
    .unwrap();
    match (endian, format) {
        (Some(endian), Some(format)) => Ok(NumberTypeNode::new(format, endian).into()),
        (_, None) => return Err(meta.error("numberTypeNode: missing format").into()),
        (None, _) => return Err(meta.error("numberTypeNode: missing endianness").into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeAttribute;
    use codama_syn_helpers::syn_build;
    use quote::quote;
    use NumberFormat::U16;

    #[test]
    fn test_type_attribute() {
        let ast = syn_build::attribute(quote! { #[node(numberTypeNode(u16, le))] });
        let attribute = NodeAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(attribute.node, NumberTypeNode::le(U16).into());
    }

    #[test]
    fn test_feature_gated_type_attribute() {
        let ast = syn_build::attribute(
            quote! { #[cfg_attr(feature = "some_feature", node(numberTypeNode(u16, le)))] },
        );
        let attribute = NodeAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(attribute.node, NumberTypeNode::le(U16).into());
    }
}
