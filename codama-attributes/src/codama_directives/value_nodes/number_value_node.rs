use crate::utils::FromMeta;
use codama_nodes::{Decimal, FloatValueNode, Integer, IntegerValueNode};
use codama_syn_helpers::{extensions::*, Meta};
use syn::{Expr, ExprLit, ExprUnary, Lit, UnOp};

impl FromMeta for IntegerValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let expr = meta.as_expr()?;
        let Some((sign, Lit::Int(int))) = signed_literal(expr) else {
            return Err(expr.error("expected an integer"));
        };
        let text = format!("{sign}{}", int.base10_digits());
        let value = match sign {
            "-" => text.parse::<i128>().map(Integer::from),
            _ => text.parse::<u128>().map(Integer::from),
        };
        value
            .map(IntegerValueNode::new)
            .map_err(|error| expr.error(error.to_string()))
    }
}

impl FromMeta for FloatValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let expr = meta.as_expr()?;
        let Some((sign, Lit::Float(float))) = signed_literal(expr) else {
            return Err(expr.error("expected a float"));
        };
        // The digits come without `_` and the type suffix, so `1_000.5f64` gives `1000.5`.
        Decimal::from_literal(&format!("{sign}{}", float.base10_digits()))
            .map(FloatValueNode::new)
            .map_err(|error| expr.error(error.to_string()))
    }
}

/// A literal with an optional leading minus sign, so `-42` gives `("-", 42)` and `42` gives `("", 42)`.
fn signed_literal(expr: &Expr) -> Option<(&str, &Lit)> {
    let (sign, expr) = match expr {
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr,
            ..
        }) => ("-", expr.as_ref()),
        _ => ("", expr),
    };
    match expr {
        Expr::Lit(ExprLit { lit, .. }) => Some((sign, lit)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DefaultValueDirective, Resolvable};

    #[test]
    fn default_values() {
        for (meta, expected) in [
            (
                syn::parse_quote! { default_value = 42 },
                IntegerValueNode::new(42u64).into(),
            ),
            (
                syn::parse_quote! { default_value = -42 },
                IntegerValueNode::new(-42i64).into(),
            ),
            (
                syn::parse_quote! { default_value = 1.5 },
                FloatValueNode::new(1.5f64).into(),
            ),
            (
                syn::parse_quote! { default_value = -1.5 },
                FloatValueNode::new(-1.5f64).into(),
            ),
        ] {
            let directive = DefaultValueDirective::parse(&meta).unwrap();
            assert_eq!(directive.node, Resolvable::Resolved(expected));
        }
    }

    #[test]
    fn integer_bounds() {
        for (meta, expected) in [
            (
                syn::parse_quote! { 340282366920938463463374607431768211455 },
                IntegerValueNode::new(u128::MAX),
            ),
            (
                syn::parse_quote! { -170141183460469231731687303715884105728 },
                IntegerValueNode::new(i128::MIN),
            ),
            (
                syn::parse_quote! { 18446744073709551616 },
                IntegerValueNode::new(u128::from(u64::MAX) + 1),
            ),
            (
                syn::parse_quote! { -9223372036854775809 },
                IntegerValueNode::new(i128::from(i64::MIN) - 1),
            ),
        ] {
            assert_eq!(IntegerValueNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn integer_out_of_range() {
        for (meta, expected) in [
            (
                syn::parse_quote! { 340282366920938463463374607431768211456 },
                "number too large to fit in target type",
            ),
            (
                syn::parse_quote! { -170141183460469231731687303715884105729 },
                "number too small to fit in target type",
            ),
        ] {
            let error = IntegerValueNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn float_keeps_the_literal_digits() {
        for (meta, expected) in [
            (syn::parse_quote! { 1.5 }, "1.5"),
            (
                syn::parse_quote! { 0.12345678901234567890 },
                "0.1234567890123456789",
            ),
            (syn::parse_quote! { 1e3 }, "1000"),
            (syn::parse_quote! { 1_000.5 }, "1000.5"),
            (syn::parse_quote! { -2.50 }, "-2.5"),
            (syn::parse_quote! { 2.5e-3f64 }, "0.0025"),
        ] {
            let node = FloatValueNode::from_meta(&meta).unwrap();
            assert_eq!(node.value.as_str(), expected);
        }
    }
}
