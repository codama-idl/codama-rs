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
    pub fn parse(ast: &'a syn::Attribute, ctx: &AttributeContext) -> syn::Result<Self> {
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        let path = attr.path();
        match (path.prefix().as_str(), path.last_str().as_str()) {
            ("" | "codama_macros" | "codama", "codama") => {
                Ok(CodamaAttribute::parse(ast, ctx)?.into())
            }
            ("", "derive") => Ok(DeriveAttribute::parse(ast)?.into()),
            ("", "repr") => Ok(ReprAttribute::parse(ast)?.into()),
            _ => Ok(UnsupportedAttribute::new(ast).into()),
        }
    }

    pub fn ast(&self) -> &syn::Attribute {
        match self {
            Attribute::Codama(a) => a.ast,
            Attribute::Derive(a) => a.ast,
            Attribute::Repr(a) => a.ast,
            Attribute::Unsupported(a) => a.ast,
        }
    }

    pub fn name(&self) -> String {
        self.ast().path().last_str()
    }
}
