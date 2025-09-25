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
                .map(|attr| Attribute::parse(attr, &ctx))
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
                match &attribute.directive {
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
            || self.has_codama_derive("CodamaType")
    }

    pub fn has_codama_derive(&self, derive: &str) -> bool {
        self.has_derive(&["", "codama", "codama_macros"], derive)
    }

    pub fn has_derive(&self, prefixes: &[&str], last: &str) -> bool {
        self.iter().filter_map(DeriveAttribute::filter).any(|attr| {
            attr.derives
                .iter()
                .any(|p| prefixes.contains(&p.prefix().as_str()) && p.last_str() == last)
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
        self.iter().filter_map(f).next()
    }

    pub fn get_last<B: 'a, F>(&'a self, f: F) -> Option<&'a B>
    where
        F: Fn(&'a Attribute<'a>) -> Option<&'a B>,
    {
        self.iter().filter_map(f).next_back()
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
