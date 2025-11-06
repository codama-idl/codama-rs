use codama_nodes::{BytesEncoding, IsAccountSigner};
use codama_syn_helpers::{extensions::*, Meta};
use syn::Expr;

pub trait FromMeta
where
    Self: Sized,
{
    fn from_meta(meta: &Meta) -> syn::Result<Self>;
}

impl FromMeta for bool {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta {
            // This is to allow attributes like `#[account(signer)]`
            // where `signer` is interpreted as `signer = true`.
            Meta::Expr(Expr::Path(_)) => Ok(true),
            _ => meta.as_expr_or_value_expr()?.as_bool(),
        }
    }
}

impl FromMeta for IsAccountSigner {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        if let Ok(value) = bool::from_meta(meta) {
            return Ok(value.into());
        }
        let expr = meta.as_expr_or_value_expr()?;
        match expr.as_string() {
            Ok(value) if value == "either" => Ok(Self::Either),
            _ => Err(expr.error("expected boolean or `\"either\"`")),
        }
    }
}

impl FromMeta for BytesEncoding {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let expr = meta.as_expr_or_value_expr()?;
        if let Ok(value) = expr.as_string() {
            if let Ok(encoding) = Self::try_from(value) {
                return Ok(encoding);
            }
        }
        Err(expr.error("expected one of: \"base16\", \"base58\", \"base64\", \"utf8\""))
    }
}
