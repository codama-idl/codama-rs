use crate::Attribute;
use codama_errors::CodamaError;
use codama_nodes::{NumberFormat, NumberTypeNode};
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct ReprAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub metas: Vec<syn::Meta>,
}

impl<'a> ReprAttribute<'a> {
    pub fn parse(ast: &'a syn::Attribute) -> syn::Result<Self> {
        let unfeatured = ast.unfeatured();
        let effective = unfeatured.as_ref().unwrap_or(ast);
        Self::parse_from(ast, effective)
    }

    /// Parse a repr attribute using the effective attribute for content extraction.
    /// `ast` is stored as the original attribute reference (for error spans).
    /// `effective` is used to parse the actual repr list.
    pub fn parse_from(ast: &'a syn::Attribute, effective: &syn::Attribute) -> syn::Result<Self> {
        let list = effective.meta.require_list()?;
        if !list.path.is_strict("repr") {
            return Err(list.path.error("expected #[repr(...)]"));
        };

        let metas = list.parse_comma_args::<syn::Meta>()?;
        Ok(Self { ast, metas })
    }

    pub fn get_number_type_node(&self) -> Option<NumberTypeNode> {
        self.metas.iter().find_map(|meta| match meta {
            syn::Meta::Path(p) => match NumberFormat::try_from(p.to_string()) {
                Ok(n) => Some(NumberTypeNode::le(n)),
                Err(_) => None,
            },
            _ => None,
        })
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a ReprAttribute<'a> {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        match attribute {
            Attribute::Repr(a) => Ok(a),
            _ => Err(CodamaError::InvalidAttribute {
                expected: "repr".to_string(),
                actual: attribute.name(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::NumberFormat::U32;
    use syn::parse_quote;

    #[test]
    fn test_repr_attribute() {
        let ast = parse_quote! { #[repr(u32, align(4))] };
        let attribute = ReprAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.metas,
            [(parse_quote! { u32 }), (parse_quote! { align(4) })]
        );
    }

    #[test]
    fn test_feature_gated_repr_attribute() {
        let ast = parse_quote! { #[cfg_attr(feature = "some_feature", repr(u32, align(4)))] };
        let attribute = ReprAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.metas,
            [(parse_quote! { u32 }), (parse_quote! { align(4) })]
        );
    }

    #[test]
    fn test_get_number_type_node() {
        let ast = parse_quote! { #[repr(u32, align(4), u64)] };
        let attribute = ReprAttribute::parse(&ast).unwrap();

        assert_eq!(
            attribute.get_number_type_node(),
            Some(NumberTypeNode::le(U32))
        );
    }
}
