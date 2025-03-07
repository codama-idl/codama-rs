use codama_nodes::IsAccountSigner;
use codama_syn_helpers::{extensions::*, Meta};
use syn::Expr;

pub trait FromMeta
where
    Self: Sized,
{
    fn from_meta(meta: &Meta) -> syn::Result<Self>;
}

impl FromMeta for String {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.as_path_value()?.value.as_expr()?.as_string()
    }
}

impl FromMeta for bool {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta {
            Meta::Expr(Expr::Path(_)) => Ok(true),
            _ => meta.as_path_value()?.value.as_expr()?.as_bool(),
        }
    }
}

impl FromMeta for IsAccountSigner {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta {
            Meta::Expr(Expr::Path(_)) => Ok(Self::True),
            _ => {
                let expr = meta.as_path_value()?.value.as_expr()?;
                if let Ok(value) = expr.as_bool() {
                    return Ok(value.into());
                }
                match expr.as_string() {
                    Ok(value) if value == "either" => Ok(Self::Either),
                    _ => Err(expr.error("expected boolean or `\"either\"`")),
                }
            }
        }
    }
}
