use super::ToTokensExtension;
use syn::{Expr, ExprLit, ExprPath, ExprUnary};

pub trait ExprExtension {
    fn get_self(&self) -> &Expr;

    /// Returns the integer value of the expression if it is a literal unsigned integer.
    fn as_unsigned_integer<T>(&self) -> syn::Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let this = self.get_self();
        match this {
            Expr::Lit(ExprLit {
                lit: syn::Lit::Int(value),
                ..
            }) => value.base10_parse::<T>(),
            _ => Err(this.error("expected an unsigned integer")),
        }
    }

    /// Returns the integer value of the expression if it is a literal signed integer.
    fn as_signed_integer<T>(&self) -> syn::Result<T>
    where
        T: std::str::FromStr + std::ops::Neg<Output = T>,
        T::Err: std::fmt::Display,
    {
        let this = self.get_self();
        let result = match this {
            Expr::Unary(ExprUnary {
                op: syn::UnOp::Neg(_),
                expr: unsigned_expr,
                ..
            }) => unsigned_expr
                .as_unsigned_integer::<T>()
                .map(|value| value.neg()),
            _ => this.as_unsigned_integer::<T>(),
        };

        result.map_err(|_| this.error("expected a signed integer"))
    }

    fn as_unsigned_float<T>(&self) -> syn::Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let this = self.get_self();
        match this {
            Expr::Lit(ExprLit {
                lit: syn::Lit::Float(value),
                ..
            }) => value.base10_parse::<T>(),
            _ => Err(this.error("expected an unsigned float")),
        }
    }

    fn as_float<T>(&self) -> syn::Result<T>
    where
        T: std::str::FromStr + std::ops::Neg<Output = T>,
        T::Err: std::fmt::Display,
    {
        let this = self.get_self();
        let result = match this {
            Expr::Unary(ExprUnary {
                op: syn::UnOp::Neg(_),
                expr: float_expr,
                ..
            }) => float_expr.as_unsigned_float::<T>().map(|value| value.neg()),
            _ => this.as_unsigned_float::<T>(),
        };

        result.map_err(|_| this.error("expected a float"))
    }

    /// Returns the string value of the expression if it is a literal string.
    fn as_string(&self) -> syn::Result<String> {
        let this = self.get_self();
        match this {
            Expr::Lit(ExprLit {
                lit: syn::Lit::Str(value),
                ..
            }) => Ok(value.value()),
            _ => Err(this.error("expected a string")),
        }
    }

    /// Returns the boolean value of the expression if it is a literal bool.
    fn as_bool(&self) -> syn::Result<bool> {
        let this = self.get_self();
        match this {
            Expr::Lit(ExprLit {
                lit: syn::Lit::Bool(value),
                ..
            }) => Ok(value.value()),
            _ => Err(this.error("expected a boolean")),
        }
    }

    /// Returns the path of the expression if it is a path.
    fn as_path(&self) -> syn::Result<&syn::Path> {
        let this = self.get_self();
        match this {
            Expr::Path(ExprPath { path, .. }) => Ok(path),
            _ => Err(this.error("expected a path")),
        }
    }
}

impl ExprExtension for Expr {
    fn get_self(&self) -> &Expr {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::*;

    #[test]
    fn as_unsigned_integer_ok() {
        let expr: Expr = syn::parse_quote! { 42 };
        let result = expr.as_unsigned_integer::<usize>().unwrap();
        assert_eq!(result, 42usize);
    }

    #[test]
    fn as_unsigned_integer_err() {
        let expr: Expr = syn::parse_quote! { -42 };
        let error = expr.as_unsigned_integer::<usize>().unwrap_err();
        assert_eq!(error.to_string(), "expected an unsigned integer");
    }

    #[test]
    fn as_signed_integer_ok() {
        let expr: Expr = syn::parse_quote! { -42 };
        let result = expr.as_signed_integer::<isize>().unwrap();
        assert_eq!(result, -42isize);
    }

    #[test]
    fn as_signed_integer_ok_with_unsigned() {
        let expr: Expr = syn::parse_quote! { 42 };
        let result = expr.as_signed_integer::<isize>().unwrap();
        assert_eq!(result, 42isize);
    }

    #[test]
    fn as_signed_integer_err() {
        let expr: Expr = syn::parse_quote! { -42.5 };
        let error = expr.as_signed_integer::<isize>().unwrap_err();
        assert_eq!(error.to_string(), "expected a signed integer");
    }

    #[test]
    fn as_float_ok() {
        let expr: Expr = syn::parse_quote! { 1.5 };
        let result = expr.as_float::<f64>().unwrap();
        assert_eq!(result, 1.5f64);
    }

    #[test]
    fn as_float_ok_negative() {
        let expr: Expr = syn::parse_quote! { -1.5 };
        let result = expr.as_float::<f64>().unwrap();
        assert_eq!(result, -1.5f64);
    }

    #[test]
    fn as_float_err() {
        let expr: Expr = syn::parse_quote! { "3.14" };
        let error = expr.as_float::<f64>().unwrap_err();
        assert_eq!(error.to_string(), "expected a float");
    }

    #[test]
    fn as_string_ok() {
        let expr: Expr = syn::parse_quote! { "hello" };
        let result = expr.as_string().unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn as_string_err() {
        let expr: Expr = syn::parse_quote! { 40 + 2 };
        let error = expr.as_string().unwrap_err();
        assert_eq!(error.to_string(), "expected a string");
    }

    #[test]
    fn as_bool_ok() {
        let expr: Expr = syn::parse_quote! { true};
        let result = expr.as_bool().unwrap();
        assert!(result);
    }

    #[test]
    fn as_bool_err() {
        let expr: Expr = syn::parse_quote! { 42 };
        let error = expr.as_bool().unwrap_err();
        assert_eq!(error.to_string(), "expected a boolean");
    }

    #[test]
    fn as_path_ok() {
        let expr: Expr = syn::parse_quote! { hello::world };
        let result = expr.as_path().unwrap().to_string();
        assert_eq!(result, "hello::world");
    }

    #[test]
    fn as_path_err() {
        let expr: Expr = syn::parse_quote! { 40 + 2 };
        let error = expr.as_path().unwrap_err();
        assert_eq!(error.to_string(), "expected a path");
    }
}
