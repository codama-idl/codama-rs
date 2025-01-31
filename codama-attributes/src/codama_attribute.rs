use crate::{utils::SetOnce, AccountDirective, AttributeContext, CodamaDirective};
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct CodamaAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub directive: CodamaDirective,
}

impl<'a> CodamaAttribute<'a> {
    pub fn parse(ast: &'a syn::Attribute, ctx: &AttributeContext) -> syn::Result<Self> {
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        // Check if the attribute is a #[codama(...)] attribute.
        let list = attr.meta.require_list()?;
        if !list.path.is_strict("codama") {
            return Err(list.path.error("expected #[codama(...)]"));
        };

        let mut directive = SetOnce::<CodamaDirective>::new("codama");
        list.each(|ref meta| directive.set(CodamaDirective::parse(meta, ctx)?, meta))?;
        Ok(Self {
            ast,
            directive: directive.take(attr)?,
        })
    }

    pub fn account(&self) -> Option<&AccountDirective> {
        match &self.directive {
            CodamaDirective::Account(a) => Some(a),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_codama_attribute() {
        let ast = parse_quote! { #[codama(type = boolean)] };
        let file = syn::File::empty();
        let ctx = AttributeContext::File(&file);
        let attribute = CodamaAttribute::parse(&ast, &ctx).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert!(matches!(attribute.directive, CodamaDirective::Type(_)));
    }

    #[test]
    fn test_feature_gated_codama_attribute() {
        let ast = parse_quote! { #[cfg_attr(feature = "some_feature", codama(type = boolean))] };
        let file = syn::File::empty();
        let ctx = AttributeContext::File(&file);
        let attribute = CodamaAttribute::parse(&ast, &ctx).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert!(matches!(attribute.directive, CodamaDirective::Type(_)));
    }
}
