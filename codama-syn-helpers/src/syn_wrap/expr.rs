use codama_errors::CodamaResult;
use quote::ToTokens;
use std::ops::Deref;

pub struct Expr<'a>(pub &'a syn::Expr);

impl<'a> Expr<'a> {
    /// Returns the integer value of the expression if it is a literal integer.
    pub fn as_literal_integer<T>(&self) -> CodamaResult<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        match self.0 {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(value),
                ..
            }) => value.base10_parse::<T>().map_err(Into::into),
            _ => Err(syn::Error::new_spanned(
                self.0,
                "Could not evaluate expression as a literal integer",
            )
            .into()),
        }
    }
}

impl ToTokens for Expr<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl Deref for Expr<'_> {
    type Target = syn::Expr;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use codama_errors::CodamaError;
    use quote::quote;

    #[test]
    fn as_literal_integer_ok() {
        let expr = syn_build::parse(quote! { 42 });
        let result = Expr(&expr).as_literal_integer::<usize>();
        assert!(matches!(result, Ok(42usize)));
    }

    #[test]
    fn as_literal_integer_err() {
        let expr = syn_build::parse(quote! { 40 + 2 });
        let result = Expr(&expr).as_literal_integer::<usize>();
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
