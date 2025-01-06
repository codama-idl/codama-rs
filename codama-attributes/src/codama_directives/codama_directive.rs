use crate::{
    AttributeContext, EncodingDirective, FixedSizeDirective, SizePrefixDirective, TypeDirective,
};
use codama_syn_helpers::{extensions::*, Meta};
use derive_more::derive::From;

use super::AccountDirective;

#[derive(Debug, PartialEq, From)]
pub enum CodamaDirective {
    // Type directives.
    Type(TypeDirective),
    Encoding(EncodingDirective),
    FixedSize(FixedSizeDirective),
    SizePrefix(SizePrefixDirective),

    // Instruction directives.
    Account(AccountDirective),
}

impl CodamaDirective {
    pub fn parse(meta: &Meta, ctx: &AttributeContext) -> syn::Result<Self> {
        let path = meta.path()?;
        match path.to_string().as_str() {
            // Type directives.
            "type" => Ok(TypeDirective::parse(meta)?.into()),
            "encoding" => Ok(EncodingDirective::parse(meta)?.into()),
            "fixed_size" => Ok(FixedSizeDirective::parse(meta)?.into()),
            "size_prefix" => Ok(SizePrefixDirective::parse(meta)?.into()),

            // Instruction directives.
            "account" => Ok(AccountDirective::parse(meta, ctx)?.into()),

            _ => Err(path.error("unrecognized codama directive")),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            // Type directives.
            CodamaDirective::Type(_) => "type",
            CodamaDirective::Encoding(_) => "encoding",
            CodamaDirective::FixedSize(_) => "fixed_size",
            CodamaDirective::SizePrefix(_) => "size_prefix",

            // Instruction directives.
            CodamaDirective::Account(_) => "account",
        }
    }
}
