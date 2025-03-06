use crate::utils::SetOnce;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct ErrorDirective {
    pub code: Option<usize>,
    pub message: Option<String>,
}

impl ErrorDirective {
    pub fn is_empty(&self) -> bool {
        self.code.is_none() && self.message.is_none()
    }
}

impl ErrorDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("error")?.as_path_list()?;
        let mut code = SetOnce::<usize>::new("code");
        let mut message = SetOnce::<String>::new("message");
        pl.each(|ref meta| match (meta.path_str().as_str(), meta) {
            ("code", _) => code.set(
                meta.as_path_value()?
                    .value
                    .as_expr()?
                    .as_literal_integer()?,
                meta,
            ),
            ("message", _) => message.set(
                meta.as_path_value()?.value.as_expr()?.as_literal_string()?,
                meta,
            ),
            (_, Meta::Expr(expr)) => {
                if let Ok(value) = expr.as_literal_integer() {
                    code.set(value, meta)
                } else if let Ok(value) = expr.as_literal_string() {
                    message.set(value, meta)
                } else {
                    Err(expr.error("expected an integer or a string"))
                }
            }
            _ => Err(meta.error("unrecognized attribute")),
        })?;
        let directive = Self {
            code: code.option(),
            message: message.option(),
        };
        if directive.is_empty() {
            return Err(pl.error("expected at least one `code` or `message` attribute"));
        }
        Ok(directive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { error(42, "my message") };
        let directive = ErrorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            ErrorDirective {
                code: Some(42),
                message: Some("my message".to_string()),
            }
        );
    }

    #[test]
    fn ok_with_explicit_labels() {
        let meta: Meta = syn::parse_quote! { error(code = 42, message = "my message") };
        let directive = ErrorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            ErrorDirective {
                code: Some(42),
                message: Some("my message".to_string()),
            }
        );
    }

    #[test]
    fn fail_if_nothing_is_provided() {
        let meta: Meta = syn::parse_quote! { error() };
        let error = ErrorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "expected at least one `code` or `message` attribute"
        );
    }
}
