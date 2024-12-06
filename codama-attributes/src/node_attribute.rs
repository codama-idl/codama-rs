use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{Endian, Node, NumberFormat, NumberTypeNode};
use codama_syn_helpers::syn_traits::*;

#[derive(Debug, PartialEq)]
pub struct NodeAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub node: Node,
}

impl<'a> TryFrom<&'a syn::Attribute> for NodeAttribute<'a> {
    type Error = CodamaError;

    fn try_from(ast: &'a syn::Attribute) -> CodamaResult<Self> {
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        // Check if the attribute is a #[node(...)] attribute.
        let list = attr.meta.require_list()?;
        if !list.path.is_strict("node") {
            return Err(syn::Error::new_spanned(&list.path, "expected #[node(...)]").into());
        };

        // Parse the node from the token stream.
        let mut node: CodamaResult<Node> =
            Err(syn::Error::new_spanned(&list.tokens, "empty node").into());
        attr.parse_nested_meta(|meta| {
            node = Self::parse_node(&meta);
            Ok(())
        })?;
        Ok(Self { ast, node: node? })
    }
}

impl<'a> NodeAttribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }

    pub fn parse_node(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node> {
        match meta.path.last_str().as_str() {
            "numberTypeNode" => parse_number_type_node_tokens(&meta),
            _ => return Err(meta.error("unrecognized node").into()),
        }
    }
}

pub fn parse_number_type_node_tokens(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node> {
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
