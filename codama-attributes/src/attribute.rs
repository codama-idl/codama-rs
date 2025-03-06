use crate::{
    AttributeContext, CodamaAttribute, DeriveAttribute, ReprAttribute, UnsupportedAttribute,
};
use codama_syn_helpers::extensions::*;
use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum Attribute<'a> {
    // E.g. `#[codama(type = number(u8))]` or `#[codama(fixed_size = 32)]`.
    Codama(CodamaAttribute<'a>),
    // E.g. `#[derive(Debug, CodamaType)]`.
    Derive(DeriveAttribute<'a>),
    // E.g. `#[repr(u32, align(8))]`.
    Repr(ReprAttribute<'a>),
    // E.g. `#[some_unsupported_attribute = 42]`.
    Unsupported(UnsupportedAttribute<'a>),
}

impl<'a> Attribute<'a> {
    pub fn parse(attr: &'a syn::Attribute, ctx: &AttributeContext) -> syn::Result<Self> {
        let path = attr.path();
        match (path.prefix().as_str(), path.last_str().as_str()) {
            ("" | "codama_macros" | "codama", "codama") => {
                Ok(CodamaAttribute::parse(attr, ctx)?.into())
            }
            ("", "derive") => Ok(DeriveAttribute::parse(attr)?.into()),
            ("", "repr") => Ok(ReprAttribute::parse(attr)?.into()),
            _ => Ok(UnsupportedAttribute::new(attr).into()),
        }
    }

    pub fn codama(&self) -> Option<&CodamaAttribute<'a>> {
        match self {
            Attribute::Codama(a) => Some(a),
            _ => None,
        }
    }

    pub fn derive(&self) -> Option<&DeriveAttribute<'a>> {
        match self {
            Attribute::Derive(a) => Some(a),
            _ => None,
        }
    }

    pub fn repr(&self) -> Option<&ReprAttribute<'a>> {
        match self {
            Attribute::Repr(a) => Some(a),
            _ => None,
        }
    }
}
