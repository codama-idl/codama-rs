use crate::{EncodingDirective, NumberDirective, TypeDirective};
use codama_syn_helpers::{extensions::*, Meta};
use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum CodamaDirective {
    // Type directives.
    Type(TypeDirective),
    Encoding(EncodingDirective),
    Number(NumberDirective),
}

impl TryFrom<&Meta> for CodamaDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let path = meta.path()?;
        match path.to_string().as_str() {
            "type" => Ok(CodamaDirective::Type(meta.try_into()?)),
            "encoding" => Ok(CodamaDirective::Encoding(meta.try_into()?)),
            "number" => Ok(CodamaDirective::Number(meta.try_into()?)),
            _ => Err(path.error("unrecognized codama directive")),
        }
    }
}
