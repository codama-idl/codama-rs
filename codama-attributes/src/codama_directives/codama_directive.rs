use crate::{
    AccountDirective, AttributeContext, DefaultValueDirective, DiscriminatorDirective,
    EncodingDirective, ErrorDirective, FixedSizeDirective, NameDirective, SizePrefixDirective,
    TypeDirective,
};
use codama_syn_helpers::{extensions::*, Meta};
use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum CodamaDirective {
    // Type directives.
    Type(TypeDirective),
    DefaultValue(DefaultValueDirective),
    Encoding(EncodingDirective),
    FixedSize(FixedSizeDirective),
    SizePrefix(SizePrefixDirective),

    // Multi-purpose directives.
    Name(NameDirective),
    Discriminator(DiscriminatorDirective),

    // Instruction directives.
    Account(AccountDirective),

    // Error directives.
    Error(ErrorDirective),
}

impl CodamaDirective {
    pub fn parse(meta: &Meta, ctx: &AttributeContext) -> syn::Result<Self> {
        let path = meta.path()?;
        match path.to_string().as_str() {
            // Type directives.
            "type" => Ok(TypeDirective::parse(meta)?.into()),
            "default_value" => Ok(DefaultValueDirective::parse(meta)?.into()),
            "encoding" => Ok(EncodingDirective::parse(meta)?.into()),
            "fixed_size" => Ok(FixedSizeDirective::parse(meta)?.into()),
            "size_prefix" => Ok(SizePrefixDirective::parse(meta)?.into()),

            // Multi-purpose directives.
            "name" => Ok(NameDirective::parse(meta)?.into()),
            "discriminator" => Ok(DiscriminatorDirective::parse(meta)?.into()),

            // Instruction directives.
            "account" => Ok(AccountDirective::parse(meta, ctx)?.into()),

            // Error directives.
            "error" => Ok(ErrorDirective::parse(meta)?.into()),

            _ => Err(path.error("unrecognized codama directive")),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            // Type directives.
            Self::Type(_) => "type",
            Self::DefaultValue(_) => "default_value",
            Self::Encoding(_) => "encoding",
            Self::FixedSize(_) => "fixed_size",
            Self::SizePrefix(_) => "size_prefix",

            // Multi-purpose directives.
            Self::Name(_) => "name",
            Self::Discriminator(_) => "discriminator",

            // Instruction directives.
            Self::Account(_) => "account",

            // Error directives.
            Self::Error(_) => "error",
        }
    }
}
