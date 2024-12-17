use crate::extensions::*;
use derive_more::From;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::discouraged::Speculative,
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
    Expr, MacroDelimiter, MetaList, Path, Token,
};

#[derive(Debug, From)]
pub enum Meta {
    /// A path — e.g. `my_attribute` or `a::b::my_attribute`.
    Path(Path),
    /// A list of Metas — e.g. `my_attribute(one, two, three)`.
    List(MetaList),
    /// A name-value pair where value is a Meta — e.g. `my_attribute = my_value`.
    Label(MetaLabel),
    /// An expression — e.g. `42`, `"hello"` or `a + b`.
    /// In case of ambiguity with `Path`, `Path` is preferred.
    Expr(Expr),
    /// A verbatim token stream — e.g. `[<=>]`.
    /// In case of ambiguity with `Expr`, `Expr` is preferred.
    Verbatim(TokenStream),
}

#[derive(Debug)]
pub struct MetaLabel {
    pub path: Path,
    pub eq_token: Token![=],
    pub value: Box<Meta>,
}

impl Meta {
    pub fn path(&self) -> syn::Result<&Path> {
        match self {
            Meta::Path(path) => Ok(path),
            Meta::List(meta) => Ok(&meta.path),
            Meta::Label(meta) => Ok(&meta.path),
            Meta::Expr(expr) => match expr {
                Expr::Path(expr) => Ok(&expr.path),
                _ => Err(expr.error("expected a path")),
            },
            Meta::Verbatim(tokens) => Err(tokens.error("expected a path")),
        }
    }

    /// Returns the path as a string if it is a valid path
    /// or an empty string if it is not a path.
    pub fn path_str(&self) -> String {
        self.path().map_or("".into(), |path| path.to_string())
    }

    pub fn is_path_or_empty_list(&self) -> bool {
        match self {
            Meta::Path(_) => true,
            Meta::List(list) if list.tokens.is_empty() => true,
            _ => false,
        }
    }

    pub fn as_path(&self) -> syn::Result<&Path> {
        let error_span = match self {
            Meta::Path(path) => return Ok(path),
            Meta::List(meta) => meta.delimiter.span().open(),
            Meta::Label(meta) => meta.eq_token.span,
            Meta::Expr(expr) => expr.span(),
            Meta::Verbatim(tokens) => tokens.span(),
        };
        Err(syn::Error::new(error_span, "unexpected token in attribute"))
    }

    pub fn as_list(&self) -> syn::Result<&MetaList> {
        match self {
            Meta::List(meta) => Ok(meta),
            Meta::Path(path) => Err(path.error(format!(
                "expected attribute arguments in parentheses: `{}(...)`",
                path.to_string(),
            ))),
            Meta::Label(meta) => Err(syn::Error::new(meta.eq_token.span, "expected `(`")),
            Meta::Expr(expr) => Err(syn::Error::new(
                expr.span(),
                "expected a path followed by `(`",
            )),
            Meta::Verbatim(tokens) => Err(syn::Error::new(
                tokens.span(),
                "expected a path followed by `(`",
            )),
        }
    }

    pub fn as_label(&self) -> syn::Result<&MetaLabel> {
        match self {
            Meta::Label(meta) => Ok(meta),
            Meta::Path(path) => Err(path.error(format!(
                "expected a value for this attribute: `{} = ...`",
                path.to_string(),
            ))),
            Meta::List(meta) => Err(syn::Error::new(
                meta.delimiter.span().open(),
                "expected `=`",
            )),
            Meta::Expr(expr) => Err(expr.error("expected a path followed by `=`")),
            Meta::Verbatim(tokens) => Err(tokens.error("expected a path followed by `=`")),
        }
    }

    pub fn as_expr(&self) -> syn::Result<&Expr> {
        let span = match self {
            Meta::Expr(expr) => return Ok(expr),
            Meta::Path(path) => path.span(),
            Meta::List(meta) => meta.delimiter.span().join(),
            Meta::Label(meta) => meta
                .path
                .span()
                .join(meta.value.span())
                .unwrap_or(meta.path.span()),
            Meta::Verbatim(tokens) => tokens.span(),
        };
        Err(syn::Error::new(span, "expected a valid expression"))
    }

    pub fn as_verbatim(&self) -> syn::Result<&TokenStream> {
        let span = match self {
            Meta::Verbatim(tokens) => return Ok(tokens),
            Meta::Path(path) => path.span(),
            Meta::List(meta) => meta.delimiter.span().join(),
            Meta::Label(meta) => meta
                .path
                .span()
                .join(meta.value.span())
                .unwrap_or(meta.path.span()),
            Meta::Expr(expr) => expr.span(),
        };
        Err(syn::Error::new(span, "expected a custom token stream"))
    }
}

impl syn::parse::Parse for Meta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        match fork.call(parse_meta_path) {
            Ok(path) => {
                if fork.peek(Paren) || fork.peek(Bracket) || fork.peek(Brace) {
                    Ok(Self::List(input.call(parse_meta_list)?))
                } else if fork.peek(Token![=]) {
                    Ok(Self::Label(input.parse()?))
                } else {
                    input.advance_to(&fork);
                    Ok(Self::Path(path))
                }
            }
            Err(_) => match fork.parse::<Expr>() {
                Ok(expr) => {
                    input.advance_to(&fork);
                    Ok(Self::Expr(expr))
                }
                _ => Ok(Self::Verbatim(input.parse_arg()?)),
            },
        }
    }
}

impl syn::parse::Parse for MetaLabel {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.call(parse_meta_path)?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

/// Parse a path without segment arguments and allowing any reserved keyword.
fn parse_meta_path(input: syn::parse::ParseStream) -> syn::Result<Path> {
    Ok(Path {
        leading_colon: input.parse()?,
        segments: {
            let mut segments = syn::punctuated::Punctuated::new();
            let ident = syn::Ident::parse_any(input)?;
            segments.push_value(syn::PathSegment::from(ident));
            while input.peek(Token![::]) {
                let punct = input.parse()?;
                segments.push_punct(punct);
                let ident = syn::Ident::parse_any(input)?;
                segments.push_value(syn::PathSegment::from(ident));
            }
            segments
        },
    })
}

/// Custom implementation of `syn::parse::Parse` for `MetaList`.
fn parse_meta_list(input: syn::parse::ParseStream) -> syn::Result<MetaList> {
    let path = input.call(parse_meta_path)?;
    let (delimiter, tokens) = input.step(|cursor| match cursor.token_tree() {
        Some((TokenTree::Group(g), rest)) => {
            let span = g.delim_span();
            let delimiter = match g.delimiter() {
                Delimiter::Parenthesis => MacroDelimiter::Paren(Paren(span)),
                Delimiter::Brace => MacroDelimiter::Brace(Brace(span)),
                Delimiter::Bracket => MacroDelimiter::Bracket(Bracket(span)),
                _ => return Err(cursor.error("expected delimiter")),
            };
            Ok(((delimiter, g.stream()), rest))
        }
        _ => Err(cursor.error("expected delimiter")),
    })?;
    Ok(MetaList {
        path,
        delimiter,
        tokens,
    })
}

impl ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Meta::Path(path) => path.to_tokens(tokens),
            Meta::List(list) => list.to_tokens(tokens),
            Meta::Label(label) => label.to_tokens(tokens),
            Meta::Expr(expr) => expr.to_tokens(tokens),
            Meta::Verbatim(verbatim) => verbatim.to_tokens(tokens),
        }
    }
}

impl ToTokens for MetaLabel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! meta {
        ($($attr:tt)*) => {{
            syn::parse_str::<Meta>(stringify!($($attr)*)).unwrap()
        }};
    }

    #[test]
    fn parse_path() {
        let meta = meta! { foo };
        let Meta::Path(path) = meta else {
            panic!("expected Meta::Path");
        };
        assert!(path.is_strict("foo"));
    }

    #[test]
    fn parse_list() {
        let meta = meta! { foo(1, 2, 3) };
        let Meta::List(list) = meta else {
            panic!("expected Meta::List");
        };
        assert!(list.path.is_strict("foo"));
        assert_eq!(list.tokens.to_string(), "1 , 2 , 3");
    }

    #[test]
    fn parse_label_with_expression() {
        let meta = meta! { foo = 42 };
        let Meta::Label(label) = meta else {
            panic!("expected Meta::Label");
        };
        assert!(label.path.is_strict("foo"));
        let expr = label.value.as_expr().unwrap();
        assert_eq!(expr.as_literal_integer::<usize>().unwrap(), 42);
    }

    #[test]
    fn parse_label_with_verbatim() {
        let meta = meta! { foo = ?what? };
        let Meta::Label(label) = meta else {
            panic!("expected Meta::Label");
        };
        assert!(label.path.is_strict("foo"));
        let value = label.value.as_verbatim().unwrap();
        assert_eq!(value.to_string(), "? what ?");
    }

    #[test]
    fn parse_label_with_list() {
        let meta = meta! { foo = bar(1, 2, 3) };
        let Meta::Label(label) = meta else {
            panic!("expected Meta::Label");
        };
        assert!(label.path.is_strict("foo"));
        let list = label.value.as_list().unwrap();
        assert!(list.path.is_strict("bar"));
        assert_eq!(list.tokens.to_string(), "1 , 2 , 3");
    }

    #[test]
    fn parse_expr() {
        let meta = meta! { "hello" };
        let Meta::Expr(expr) = meta else {
            panic!("expected Meta::Expr");
        };
        assert_eq!(expr.to_token_stream().to_string(), "\"hello\"");
    }

    #[test]
    fn parse_verbatim() {
        let meta = meta! { [==> 42 <==] };
        let Meta::Verbatim(verbatim) = meta else {
            panic!("expected Meta::Verbatim");
        };
        assert_eq!(verbatim.to_string(), "[==> 42 <==]");
    }

    #[test]
    fn parse_verbatim_list() -> syn::Result<()> {
        let meta = meta! { foo([==> 1 <==], [==> 2 <==]) };
        let list = meta.as_list()?;
        assert_eq!(list.path.to_string(), "foo");
        let metas = list.parse_metas()?;
        assert_eq!(metas.len(), 2);
        assert_eq!(metas[0].as_verbatim()?.to_string(), "[==> 1 <==]");
        assert_eq!(metas[1].as_verbatim()?.to_string(), "[==> 2 <==]");
        Ok(())
    }

    #[test]
    fn path() -> syn::Result<()> {
        assert_eq!(meta! { foo }.path()?.to_string(), "foo");
        assert_eq!(meta! { foo(42) }.path()?.to_string(), "foo");
        assert_eq!(meta! { foo = 42 }.path()?.to_string(), "foo");
        assert_eq!(meta! { foo = bar(42) }.path()?.to_string(), "foo");
        assert_eq!(
            meta! { [verbatim] }.path().unwrap_err().to_string(),
            "expected a path"
        );
        Ok(())
    }

    #[test]
    fn is_path_or_empty_list() {
        assert!(meta! { foo }.is_path_or_empty_list());
        assert!(meta! { some_node }.is_path_or_empty_list());
        assert!(meta! { foo() }.is_path_or_empty_list());
        assert!(meta! { some_node() }.is_path_or_empty_list());
        assert!(!meta! { foo = 42 }.is_path_or_empty_list());
        assert!(!meta! { foo = bar(1, 2, 3) }.is_path_or_empty_list());
        assert!(!meta! { foo(answer = 42) }.is_path_or_empty_list());
        assert!(!meta! { some_node(hello) }.is_path_or_empty_list());
        assert!(!meta! { 42 }.is_path_or_empty_list());
        assert!(!meta! { [verbatim] }.is_path_or_empty_list());
    }

    #[test]
    fn as_path() {
        assert_eq!(meta! { foo }.as_path().unwrap().to_string(), "foo");

        let msg = "unexpected token in attribute";
        assert_eq!(meta! { foo(42) }.as_path().unwrap_err().to_string(), msg);
        assert_eq!(meta! { foo = 42 }.as_path().unwrap_err().to_string(), msg);
        assert_eq!(
            meta! { foo = bar(42) }.as_path().unwrap_err().to_string(),
            msg
        );
        assert_eq!(meta! { [verbatim] }.as_path().unwrap_err().to_string(), msg);
    }

    #[test]
    fn as_list() {
        let meta = meta! { foo(1, 2, 3) };
        let list = meta.as_list().unwrap();
        assert!(list.path.is_strict("foo"));
        assert_eq!(list.tokens.to_string(), "1 , 2 , 3");

        assert_eq!(
            meta! { foo }.as_list().unwrap_err().to_string(),
            "expected attribute arguments in parentheses: `foo(...)`"
        );
        assert_eq!(
            meta! { foo = 42 }.as_list().unwrap_err().to_string(),
            "expected `(`"
        );
        assert_eq!(
            meta! { foo = bar(42) }.as_list().unwrap_err().to_string(),
            "expected `(`"
        );
        assert_eq!(
            meta! { [verbatim] }.as_list().unwrap_err().to_string(),
            "expected a path followed by `(`"
        );
    }
}
