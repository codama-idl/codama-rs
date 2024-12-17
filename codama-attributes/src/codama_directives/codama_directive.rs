use crate::{NumberDirective, StringDirective, TypeDirective};
use codama_syn_helpers::{extensions::*, Meta};
use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum CodamaDirective {
    Type(TypeDirective),
    Number(NumberDirective),
    String(StringDirective),
}

impl TryFrom<&Meta> for CodamaDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let path = meta.path()?;
        match path.to_string().as_str() {
            "type" => Ok(CodamaDirective::Type(meta.try_into()?)),
            "number" => Ok(CodamaDirective::Number(meta.try_into()?)),
            "string" => Ok(CodamaDirective::String(meta.try_into()?)),
            _ => Err(path.error("unrecognized codama directive")),
        }
    }
}
