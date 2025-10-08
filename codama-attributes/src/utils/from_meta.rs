use codama_nodes::{BytesEncoding, IsAccountSigner};
use codama_syn_helpers::{extensions::*, Meta, PathList};
use syn::Expr;

pub trait FromMeta
where
    Self: Sized,
{
    fn from_meta(meta: &Meta) -> syn::Result<Self>;
}

impl FromMeta for String {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        get_expr_from_meta(meta)?.as_string()
    }
}

impl FromMeta for bool {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta {
            // This is to allow attributes like `#[account(signer)]`
            // where `signer` is interpreted as `signer = true`.
            Meta::Expr(Expr::Path(_)) => Ok(true),
            _ => get_expr_from_meta(meta)?.as_bool(),
        }
    }
}

impl FromMeta for usize {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        get_expr_from_meta(meta)?.as_unsigned_integer()
    }
}

impl FromMeta for IsAccountSigner {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        if let Ok(value) = bool::from_meta(meta) {
            return Ok(value.into());
        }
        let expr = get_expr_from_meta(meta)?;
        match expr.as_string() {
            Ok(value) if value == "either" => Ok(Self::Either),
            _ => Err(expr.error("expected boolean or `\"either\"`")),
        }
    }
}

impl FromMeta for BytesEncoding {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let expr = get_expr_from_meta(meta)?;
        if let Ok(value) = expr.as_string() {
            if let Ok(encoding) = Self::try_from(value) {
                return Ok(encoding);
            }
        }
        Err(expr.error("expected one of: \"base16\", \"base58\", \"base64\", \"utf8\""))
    }
}

pub fn get_expr_from_meta(meta: &Meta) -> syn::Result<&Expr> {
    match meta {
        Meta::Expr(expr) => Ok(expr),
        _ => meta.as_path_value()?.value.as_expr(),
    }
}

pub fn get_expr_from_meta_with_path_lists_as_arrays(meta: &Meta) -> syn::Result<Expr> {
    match meta {
        Meta::Expr(expr) => Ok(expr.clone()),
        Meta::PathList(pl) => Ok(get_array_expr_from_path_list(pl)?.into()),
        _ => meta.as_path_value()?.value.as_expr().cloned(),
    }
}

pub fn get_array_expr_from_path_list(pl: &PathList) -> syn::Result<syn::ExprArray> {
    let syn::MacroDelimiter::Bracket(bracket_token) = pl.delimiter else {
        return Err(pl.error("expected an array delimited with `[]`"));
    };

    Ok(syn::ExprArray {
        attrs: vec![],
        bracket_token,
        elems: pl.as_meta_list().parse_args_with(
            syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated,
        )?,
    })
}
