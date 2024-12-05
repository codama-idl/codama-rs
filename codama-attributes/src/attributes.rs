use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::Attribute;
use codama_errors::{CodamaError, CodamaResult};

#[derive(Debug, PartialEq)]
pub struct Attributes<'a>(pub Vec<Attribute<'a>>);

impl<'a> Attributes<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attrs: T) -> CodamaResult<Self> {
        attrs.try_into()
    }
}

impl<'a> TryFrom<Vec<&'a syn::Attribute>> for Attributes<'a> {
    type Error = CodamaError;

    fn try_from(attrs: Vec<&'a syn::Attribute>) -> CodamaResult<Self> {
        let attributes = attrs
            .iter()
            .map(|attr| Attribute::parse(attr))
            .collect::<CodamaResult<Vec<_>>>()?;
        Ok(Self(attributes))
    }
}

impl<'a> TryFrom<&'a Vec<syn::Attribute>> for Attributes<'a> {
    type Error = CodamaError;

    fn try_from(attrs: &'a Vec<syn::Attribute>) -> CodamaResult<Self> {
        let attributes = attrs
            .iter()
            .map(Attribute::parse)
            .collect::<CodamaResult<Vec<_>>>()?;
        Ok(Self(attributes))
    }
}

impl<'a> Deref for Attributes<'a> {
    type Target = Vec<Attribute<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Attributes<'a> {
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

impl<'a> IndexMut<usize> for Attributes<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
