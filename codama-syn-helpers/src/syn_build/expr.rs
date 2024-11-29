use codama_errors::CodamaResult;
use proc_macro2::TokenStream;

/// E.g. `40 + 2`
pub fn try_expr(tt: TokenStream) -> CodamaResult<syn::Expr> {
    syn::parse2::<syn::Expr>(tt).map_err(|e| e.into())
}

/// E.g. `40 + 2`
pub fn expr(tt: TokenStream) -> syn::Expr {
    try_expr(tt).unwrap()
}

#[cfg(test)]
mod tests {
    use codama_errors::CodamaError;
    use quote::quote;

    use super::*;

    #[test]
    fn expr_ok() {
        let result = try_expr(quote! { (40 + 2) * 2 });
        assert!(matches!(result, Ok(syn::Expr::Binary(_))));
    }

    #[test]
    fn expr_err() {
        let result = try_expr(quote! { struct Foo {} });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
