use crate::utils::SetOnce;
use codama_errors::CodamaResult;
use codama_nodes::{Endian, Node, NumberFormat, NumberTypeNode};
use codama_syn_helpers::syn_traits::*;

pub fn number_type_node(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeAttribute;
    use codama_syn_helpers::syn_build;
    use quote::quote;
    use NumberFormat::U16;

    pub fn get_node(tt: proc_macro2::TokenStream) -> CodamaResult<Node> {
        let ast = syn_build::attribute(tt);
        let attribute = NodeAttribute::parse(&ast)?;
        Ok(attribute.node)
    }

    #[test]
    fn ok() {
        assert_eq!(
            get_node(quote! { #[node(numberTypeNode(u16, le))] }).unwrap(),
            NumberTypeNode::le(U16).into()
        );
    }
}