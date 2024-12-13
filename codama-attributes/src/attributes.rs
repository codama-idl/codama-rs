use crate::Attribute;
use codama_errors::IteratorCombineErrors;
use codama_syn_helpers::extensions::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq)]
pub struct Attributes<'a>(pub Vec<Attribute<'a>>);

impl Attributes<'_> {
    pub fn parse<T: TryInto<Self, Error = syn::Error>>(attrs: T) -> syn::Result<Self> {
        attrs.try_into()
    }

    pub fn has_derive(&self, derive: &str) -> bool {
        self.iter().any(|attr| match attr {
            Attribute::Derive(a) => a.derives.iter().any(|p| p.is_strict(derive)),
            _ => false,
        })
    }
}

impl<'a> TryFrom<Vec<&'a syn::Attribute>> for Attributes<'a> {
    type Error = syn::Error;

    fn try_from(attrs: Vec<&'a syn::Attribute>) -> syn::Result<Self> {
        let attributes = attrs
            .iter()
            .map(|attr: &&syn::Attribute| Attribute::parse(*attr))
            .collect_and_combine_errors()?;

        Ok(Self(attributes))
    }
}

impl<'a> TryFrom<&'a Vec<syn::Attribute>> for Attributes<'a> {
    type Error = syn::Error;

    fn try_from(attrs: &'a Vec<syn::Attribute>) -> syn::Result<Self> {
        let attributes = attrs
            .iter()
            .map(Attribute::parse)
            .collect::<syn::Result<Vec<_>>>()?;
        Ok(Self(attributes))
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
