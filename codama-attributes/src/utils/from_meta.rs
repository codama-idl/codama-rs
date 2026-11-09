use codama_nodes::{BytesEncoding, Docs, Identifier, IsSigner, OptionalAccountStrategy};
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

/// Parses a string identifier, either positional (`"amount"`) or as a value (`identifier = "amount"`).
/// An invalid identifier points at the string.
impl FromMeta for Identifier {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let value = match meta {
            Meta::PathValue(pv) => &pv.value,
            meta => meta,
        };
        let expr = value
            .as_expr()
            .map_err(|_| value.error("expected a string"))?;
        expr.as_string()?.parse().map_err(|e| expr.error(e))
    }
}

impl FromMeta for IsSigner {
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

impl FromMeta for OptionalAccountStrategy {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let expr = meta.as_expr_or_value_expr()?;
        let path = expr.as_path()?;
        match path.to_string().as_str() {
            "omitted" => Ok(Self::Omitted),
            "program_id" => Ok(Self::ProgramId),
            _ => Err(path.error("expected `omitted` or `program_id`")),
        }
    }
}

impl FromMeta for Docs {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta {
            Meta::PathList(pl) if pl.eq_token.is_some() => {
                let strings: Vec<String> = pl
                    .as_meta_list()
                    .parse_comma_args::<syn::LitStr>()?
                    .into_iter()
                    .map(|lit| lit.value())
                    .collect();
                Ok(strings.into())
            }
            Meta::PathValue(pv) => pv.value.as_expr()?.as_string().map(|s| vec![s].into()),
            _ => Err(meta.error("expected a string or array of strings")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_from_a_positional_string() {
        let meta: Meta = syn::parse_quote! { "amount" };
        assert_eq!(
            Identifier::from_meta(&meta).unwrap(),
            Identifier::from("amount")
        );
    }

    #[test]
    fn identifier_from_a_value() {
        let meta: Meta = syn::parse_quote! { identifier = "amount" };
        assert_eq!(
            Identifier::from_meta(&meta).unwrap(),
            Identifier::from("amount")
        );
    }

    #[test]
    fn identifier_rejects_a_non_string() {
        for meta in [
            syn::parse_quote! { identifier = amount },
            syn::parse_quote! { identifier = 42 },
            syn::parse_quote! { identifier = amount(1, 2) },
            syn::parse_quote! { amount(1, 2) },
        ] {
            let error = Identifier::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), "expected a string");
        }
    }

    #[test]
    fn identifier_rejects_an_invalid_identifier() {
        let meta: Meta = syn::parse_quote! { identifier = "1st" };
        let error = Identifier::from_meta(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Invalid identifier `1st`: use letters, digits and underscores, not starting with a digit"
        );
    }
}
