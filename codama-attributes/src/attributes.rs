use crate::{
    Attribute, AttributeContext, CodamaAttribute, CodamaDirective, DeriveAttribute, TryFromFilter,
};
use codama_errors::IteratorCombineErrors;
use codama_syn_helpers::extensions::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq)]
pub struct Attributes<'a>(pub Vec<Attribute<'a>>);

impl<'a> Attributes<'a> {
    pub fn parse(attrs: &'a [syn::Attribute], ctx: AttributeContext<'a>) -> syn::Result<Self> {
        let attributes = Self(
            attrs
                .iter()
                // Expand multi-attr cfg_attr into (ast, effective) pairs
                .flat_map(|ast| {
                    let inners = ast.unfeatured_all();
                    if inners.len() <= 1 {
                        // Not a multi-attr cfg_attr - use standard parsing
                        let unfeatured = ast.unfeatured();
                        let effective = unfeatured.unwrap_or_else(|| (*ast).clone());
                        vec![(ast, effective)]
                    } else {
                        // Multi-attr cfg_attr - expand each inner attribute
                        inners.into_iter().map(|inner| (ast, inner)).collect()
                    }
                })
                .map(|(ast, effective)| Attribute::parse_from(ast, &effective, &ctx))
                .collect_and_combine_errors()?,
        );
        attributes.validate_codama_type_attributes()?;
        Ok(attributes)
    }

    pub fn validate_codama_type_attributes(&self) -> syn::Result<()> {
        let mut errors = Vec::<syn::Error>::new();
        let mut has_seen_type = false;

        for attribute in self.0.iter().rev() {
            if let Attribute::Codama(attribute) = attribute {
                match attribute.directive.as_ref() {
                    CodamaDirective::Type(_) if !has_seen_type => has_seen_type = true,
                    CodamaDirective::Type(_)
                    | CodamaDirective::Encoding(_)
                    | CodamaDirective::FixedSize(_)
                        if has_seen_type =>
                    {
                        errors.push(syn::Error::new_spanned(
                            attribute.ast,
                            "This attribute is overridden by a `#[codama(type = ...)]` attribute below",
                        ));
                    }
                    _ => {}
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            // Combine all errors into one
            let mut combined_error = errors.remove(0);
            for error in errors {
                combined_error.combine(error);
            }
            Err(combined_error)
        }
    }

    pub fn has_any_codama_derive(&self) -> bool {
        self.has_codama_derive("CodamaAccount")
            || self.has_codama_derive("CodamaAccounts")
            || self.has_codama_derive("CodamaErrors")
            || self.has_codama_derive("CodamaInstruction")
            || self.has_codama_derive("CodamaInstructions")
            || self.has_codama_derive("CodamaPda")
            || self.has_codama_derive("CodamaType")
    }

    pub fn has_codama_derive(&self, derive: &str) -> bool {
        self.has_derive(&["", "codama", "codama_macros"], derive)
    }

    pub fn has_derive(&self, prefixes: &[&str], last: &str) -> bool {
        self.iter().filter_map(DeriveAttribute::filter).any(|attr| {
            attr.derives
                .iter()
                .any(|p| p.last_str() == last && prefixes.contains(&p.prefix().as_str()))
        })
    }

    pub fn has_codama_attribute(&self, name: &str) -> bool {
        self.iter()
            .filter_map(CodamaAttribute::filter)
            .any(|a| a.directive.name() == name)
    }

    pub fn get_all<B: 'a, F>(&'a self, f: F) -> Vec<&'a B>
    where
        F: Fn(&'a Attribute<'a>) -> Option<&'a B>,
    {
        self.iter().filter_map(f).collect()
    }

    pub fn get_first<B: 'a, F>(&'a self, f: F) -> Option<&'a B>
    where
        F: Fn(&'a Attribute<'a>) -> Option<&'a B>,
    {
        self.iter().find_map(f)
    }

    pub fn get_last<B: 'a, F>(&'a self, f: F) -> Option<&'a B>
    where
        F: Fn(&'a Attribute<'a>) -> Option<&'a B>,
    {
        self.iter().rev().find_map(f)
    }
}

impl<'a> Deref for Attributes<'a> {
    type Target = Vec<Attribute<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Attributes<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> AsRef<[Attribute<'a>]> for Attributes<'a> {
    fn as_ref(&self) -> &[Attribute<'a>] {
        &self.0
    }
}

impl<'a> AsMut<[Attribute<'a>]> for Attributes<'a> {
    fn as_mut(&mut self) -> &mut [Attribute<'a>] {
        &mut self.0
    }
}

impl<'a> Index<usize> for Attributes<'a> {
    type Output = Attribute<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Attributes<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn file_ctx() -> syn::File {
        syn::File::empty()
    }

    #[test]
    fn parse_single_codama_attr() {
        let file = file_ctx();
        let attrs: Vec<syn::Attribute> = vec![parse_quote! { #[codama(type = boolean)] }];
        let ctx = AttributeContext::File(&file);
        let attributes = Attributes::parse(&attrs, ctx).unwrap();

        assert_eq!(attributes.len(), 1);
        assert!(matches!(&attributes[0], Attribute::Codama(_)));
    }

    #[test]
    fn parse_feature_gated_single_codama_attr() {
        let file = file_ctx();
        let attrs: Vec<syn::Attribute> =
            vec![parse_quote! { #[cfg_attr(feature = "codama", codama(type = boolean))] }];
        let ctx = AttributeContext::File(&file);
        let attributes = Attributes::parse(&attrs, ctx).unwrap();

        assert_eq!(attributes.len(), 1);
        assert!(matches!(&attributes[0], Attribute::Codama(_)));
    }

    #[test]
    fn parse_multi_attr_cfg_attr_expands_all() {
        let file = file_ctx();
        // This is the bug scenario: multi-attr cfg_attr should expand to multiple attributes
        let attrs: Vec<syn::Attribute> = vec![parse_quote! {
            #[cfg_attr(feature = "codama", codama(type = boolean), codama(name = "foo"))]
        }];
        let ctx = AttributeContext::File(&file);
        let attributes = Attributes::parse(&attrs, ctx).unwrap();

        // Should have 2 attributes from the expansion
        assert_eq!(attributes.len(), 2);
        assert!(matches!(&attributes[0], Attribute::Codama(_)));
        assert!(matches!(&attributes[1], Attribute::Codama(_)));

        // Verify the directives
        if let Attribute::Codama(attr) = &attributes[0] {
            assert!(matches!(attr.directive.as_ref(), CodamaDirective::Type(_)));
        }
        if let Attribute::Codama(attr) = &attributes[1] {
            assert!(matches!(attr.directive.as_ref(), CodamaDirective::Name(_)));
        }
    }

    #[test]
    fn parse_multi_attr_cfg_attr_mixed_types() {
        let file = file_ctx();
        // cfg_attr with mixed attribute types: derive, codama
        let attrs: Vec<syn::Attribute> = vec![parse_quote! {
            #[cfg_attr(feature = "x", derive(Debug), codama(type = boolean))]
        }];
        let ctx = AttributeContext::File(&file);
        let attributes = Attributes::parse(&attrs, ctx).unwrap();

        // Should have 2 attributes: Derive and Codama
        assert_eq!(attributes.len(), 2);
        assert!(matches!(&attributes[0], Attribute::Derive(_)));
        assert!(matches!(&attributes[1], Attribute::Codama(_)));
    }

    #[test]
    fn parse_multi_attr_cfg_attr_preserves_order() {
        let file = file_ctx();
        let attrs: Vec<syn::Attribute> = vec![parse_quote! {
            #[cfg_attr(feature = "codama", codama(name = "first"), codama(name = "second"), codama(name = "third"))]
        }];
        let ctx = AttributeContext::File(&file);
        let attributes = Attributes::parse(&attrs, ctx).unwrap();

        assert_eq!(attributes.len(), 3);

        // All should be Name directives in order
        let names: Vec<_> = attributes
            .iter()
            .filter_map(CodamaAttribute::filter)
            .filter_map(|a| {
                if let CodamaDirective::Name(n) = a.directive.as_ref() {
                    Some(n.name.as_ref().to_string())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(names, vec!["first", "second", "third"]);
    }

    #[test]
    fn parse_multiple_separate_cfg_attr_and_multi_attr() {
        let file = file_ctx();
        // Mix of separate attrs and multi-attr cfg_attr
        let attrs: Vec<syn::Attribute> = vec![
            parse_quote! { #[derive(Clone)] },
            parse_quote! { #[cfg_attr(feature = "x", codama(name = "a"), codama(name = "b"))] },
            parse_quote! { #[codama(type = boolean)] },
        ];
        let ctx = AttributeContext::File(&file);
        let attributes = Attributes::parse(&attrs, ctx).unwrap();

        // Should have 4 attributes: Derive, 2 Codama from cfg_attr, 1 Codama bare
        assert_eq!(attributes.len(), 4);
        assert!(matches!(&attributes[0], Attribute::Derive(_)));
        assert!(matches!(&attributes[1], Attribute::Codama(_)));
        assert!(matches!(&attributes[2], Attribute::Codama(_)));
        assert!(matches!(&attributes[3], Attribute::Codama(_)));
    }
}
