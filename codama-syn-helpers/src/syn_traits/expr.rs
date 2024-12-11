use super::ToTokens as _;

pub trait Expr {
    fn get_self(&self) -> &syn::Expr;

    /// Returns the integer value of the expression if it is a literal integer.
    fn as_literal_integer<T>(&self) -> syn::Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let this = self.get_self();
        match this {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(value),
                ..
            }) => value.base10_parse::<T>(),
            _ => Err(this.error("expected a literal integer")),
        }
    }

    /// Returns the string value of the expression if it is a literal string.
    fn as_literal_string(&self) -> syn::Result<String> {
        let this = self.get_self();
        match this {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(value),
                ..
            }) => Ok(value.value()),
            _ => Err(this.error("expected a literal string")),
        }
    }

    /// Returns the path of the expression if it is a path.
    fn as_path(&self) -> syn::Result<&syn::Path> {
        let this = self.get_self();
        match this {
            syn::Expr::Path(syn::ExprPath { path, .. }) => Ok(path),
            _ => Err(this.error("expected a path")),
        }
    }
}

impl Expr for syn::Expr {
    fn get_self(&self) -> &syn::Expr {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{syn_build, syn_traits::Path};
    use quote::quote;

    #[test]
    fn as_literal_integer_ok() {
        let expr: syn::Expr = syn_build::parse(quote! { 42 });
        let result = expr.as_literal_integer::<usize>();
        assert!(matches!(result, Ok(42usize)));
    }

    #[test]
    fn as_literal_integer_err() {
        let expr: syn::Expr = syn_build::parse(quote! { 40 + 2 });
        let error = expr.as_literal_integer::<usize>().unwrap_err();
        assert_eq!(error.to_string(), "expected a literal integer");
    }

    #[test]
    fn as_literal_string_ok() {
        let expr: syn::Expr = syn_build::parse(quote! { "hello" });
        let result = expr.as_literal_string().unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn as_literal_string_err() {
        let expr: syn::Expr = syn_build::parse(quote! { 40 + 2 });
        let error = expr.as_literal_string().unwrap_err();
        assert_eq!(error.to_string(), "expected a literal string");
    }

    #[test]
    fn as_path_ok() {
        let expr: syn::Expr = syn_build::parse(quote! { hello::world });
        let result = expr.as_path().unwrap().to_string();
        assert_eq!(result, "hello::world");
    }

    #[test]
    fn as_path_err() {
        let expr: syn::Expr = syn_build::parse(quote! { 40 + 2 });
        let error = expr.as_path().unwrap_err();
        assert_eq!(error.to_string(), "expected a path");
    }
}
