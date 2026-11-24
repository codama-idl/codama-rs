use crate::{
    Attribute, AttributeContext, CodamaDirective, DeriveAttribute, ExportDirective, TryFromFilter,
};
use codama_errors::IteratorCombineErrors;
use codama_syn_helpers::extensions::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// The derive macros of `codama-macros` that turn an item into a Codama node.
const CODAMA_DERIVES: [&str; 9] = [
    "CodamaAccount",
    "CodamaAccounts",
    "CodamaErrors",
    "CodamaEvent",
    "CodamaEvents",
    "CodamaInstruction",
    "CodamaInstructions",
    "CodamaPda",
    "CodamaType",
];

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
        attributes.validate_single_codama_derive()?;
        Ok(attributes)
    }

    /// An item derives at most one Codama item, since each derive decides what the item becomes.
    /// E.g. `#[derive(CodamaType, CodamaAccount)]` would make it both a defined type and an account.
    fn validate_single_codama_derive(&self) -> syn::Result<()> {
        let mut derives = self.codama_derives().into_iter();
        let Some((first, _)) = derives.next() else {
            return Ok(());
        };
        derives
            .map(|(derive, ast)| {
                syn::Error::new_spanned(
                    ast,
                    format!("`{derive}` conflicts with `{first}`: an item derives at most one Codama item"),
                )
            })
            .reduce(|mut combined, error| {
                combined.combine(error);
                combined
            })
            .map_or(Ok(()), Err)
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

    /// The Codama derives of the item, such as `CodamaAccount`, each with the path that names it,
    /// in the order they are written.
    pub fn codama_derives(&self) -> Vec<(&'static str, &syn::Path)> {
        self.iter()
            .filter_map(DeriveAttribute::filter)
            .flat_map(|attr| attr.derives.iter())
            .filter_map(|path| {
                let is_codama = ["", "codama", "codama_macros"].contains(&path.prefix().as_str());
                let derive = CODAMA_DERIVES
                    .iter()
                    .find(|derive| path.last_str() == **derive)?;
                is_codama.then_some((*derive, path))
            })
            .collect()
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

    pub fn has_codama_export(&self, derive: &str) -> bool {
        self.iter()
            .filter_map(ExportDirective::filter)
            .any(|e| e.derive == derive)
    }

    /// Whether any attribute passes the filter, such as `attributes.has(SkipDirective::filter)`.
    pub fn has<B: 'a, F>(&'a self, f: F) -> bool
    where
        F: Fn(&'a Attribute<'a>) -> Option<&'a B>,
    {
        self.iter().any(|attribute| f(attribute).is_some())
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
    use crate::CodamaAttribute;
    use syn::parse_quote;

    fn file_ctx() -> syn::File {
        syn::File::empty()
    }

    #[test]
    fn rejects_items_deriving_several_codama_items() {
        let file = file_ctx();
        let attrs: Vec<syn::Attribute> = vec![
            parse_quote! { #[derive(Debug, CodamaType)] },
            parse_quote! { #[derive(codama::CodamaAccount, CodamaPda)] },
        ];
        let error = Attributes::parse(&attrs, AttributeContext::File(&file)).unwrap_err();
        let messages: Vec<_> = error.into_iter().map(|e| e.to_string()).collect();
        assert_eq!(
            messages,
            [
                "`CodamaAccount` conflicts with `CodamaType`: an item derives at most one Codama item",
                "`CodamaPda` conflicts with `CodamaType`: an item derives at most one Codama item",
            ]
        );
    }

    #[test]
    fn accepts_a_single_codama_derive_among_other_derives() {
        let file = file_ctx();
        let attrs: Vec<syn::Attribute> =
            vec![parse_quote! { #[derive(Debug, Clone, CodamaAccount, serde::Serialize)] }];
        let attributes = Attributes::parse(&attrs, AttributeContext::File(&file)).unwrap();
        assert_eq!(attributes.codama_derives().len(), 1);
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
            #[cfg_attr(feature = "codama", codama(type = boolean), codama(identifier = "foo"))]
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
            assert!(matches!(
                attr.directive.as_ref(),
                CodamaDirective::Identifier(_)
            ));
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
            #[cfg_attr(feature = "codama", codama(identifier = "first"), codama(identifier = "second"), codama(identifier = "third"))]
        }];
        let ctx = AttributeContext::File(&file);
        let attributes = Attributes::parse(&attrs, ctx).unwrap();

        assert_eq!(attributes.len(), 3);

        // All should be Identifier directives in order
        let names: Vec<_> = attributes
            .iter()
            .filter_map(CodamaAttribute::filter)
            .filter_map(|a| {
                if let CodamaDirective::Identifier(n) = a.directive.as_ref() {
                    Some(n.identifier.as_ref().to_string())
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
            parse_quote! { #[cfg_attr(feature = "x", codama(identifier = "a"), codama(identifier = "b"))] },
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

    #[test]
    fn has_finds_a_directive_by_type() {
        let file = file_ctx();
        let attrs: Vec<syn::Attribute> = vec![
            parse_quote! { #[codama(type = boolean)] },
            parse_quote! { #[codama(skip)] },
        ];
        let attributes = Attributes::parse(&attrs, AttributeContext::File(&file)).unwrap();

        assert!(attributes.has(crate::SkipDirective::filter));
        assert!(attributes.has(crate::TypeDirective::filter));
        assert!(!attributes.has(crate::SeedDirective::filter));
    }
}
