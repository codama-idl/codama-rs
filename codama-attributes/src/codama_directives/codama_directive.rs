use crate::{
    AccountDirective, AttributeContext, DefaultValueDirective, DiscriminatorDirective,
    EncodingDirective, EnumDiscriminatorDirective, ErrorDirective, FieldDirective,
    FixedSizeDirective, NameDirective, SizePrefixDirective, TypeDirective,
};
use codama_syn_helpers::{extensions::*, Meta};
use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum CodamaDirective {
    // Type directives.
    Type(TypeDirective),
    DefaultValue(DefaultValueDirective),
    Encoding(EncodingDirective),
    Field(FieldDirective),
    FixedSize(FixedSizeDirective),
    SizePrefix(SizePrefixDirective),

    // Multi-purpose directives.
    Discriminator(DiscriminatorDirective),
    EnumDiscriminator(EnumDiscriminatorDirective),
    Name(NameDirective),

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
            "field" => Ok(FieldDirective::parse(meta)?.into()),
            "fixed_size" => Ok(FixedSizeDirective::parse(meta)?.into()),
            "size_prefix" => Ok(SizePrefixDirective::parse(meta)?.into()),

            // Multi-purpose directives.
            "discriminator" => Ok(DiscriminatorDirective::parse(meta)?.into()),
            "enum_discriminator" => Ok(EnumDiscriminatorDirective::parse(meta)?.into()),
            "name" => Ok(NameDirective::parse(meta)?.into()),

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
            Self::Field(_) => "field",
            Self::FixedSize(_) => "fixed_size",
            Self::SizePrefix(_) => "size_prefix",

            // Multi-purpose directives.
            Self::Discriminator(_) => "discriminator",
            Self::EnumDiscriminator(_) => "enum_discriminator",
            Self::Name(_) => "name",

            // Instruction directives.
            Self::Account(_) => "account",

            // Error directives.
            Self::Error(_) => "error",
        }
    }
}
