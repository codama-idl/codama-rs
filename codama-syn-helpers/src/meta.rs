use crate::extensions::*;
use derive_more::From;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::discouraged::Speculative,
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
    Expr, MacroDelimiter, MetaList, MetaNameValue, Path, Token,
};

#[derive(Debug, From)]
pub enum Meta {
    /// A path — e.g. `my_attribute` or `a::b::my_attribute`.
    Path(Path),
    /// A list of Metas — e.g. `my_attribute(one, two, three)`.
    List(MetaList),
    /// A name-list pair where list is a list of Metas — e.g. `my_attribute = my_value(one, two, three)`.
    NameList(MetaNameList),
    /// A name-value pair where value is an expression — e.g. `my_attribute = 42`.
    /// In case of ambiguity with `NameList`, `NameList` is preferred.
    NameValue(MetaNameValue),
    /// An expression — e.g. `42`, `"hello"` or `a + b`.
    /// In case of ambiguity with `Path`, `Path` is preferred.
    Expr(Expr),
    /// A verbatim token stream — e.g. `[<=>]`.
    /// In case of ambiguity with `Expr`, `Expr` is preferred.
    Verbatim(TokenStream),
}

#[derive(Debug)]
pub struct MetaNameList {
    pub path: Path,
    pub eq_token: Token![=],
    pub list: MetaList,
}

impl Meta {
    pub fn path(&self) -> syn::Result<&Path> {
        match self {
            Meta::Path(path) => Ok(path),
            Meta::List(meta) => Ok(&meta.path),
            Meta::NameValue(meta) => Ok(&meta.path),
            Meta::NameList(meta) => Ok(&meta.path),
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
            Meta::NameValue(meta) => meta.eq_token.span,
            Meta::NameList(meta) => meta.eq_token.span,
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
            Meta::NameValue(meta) => Err(syn::Error::new(meta.eq_token.span, "expected `(`")),
            Meta::NameList(meta) => Err(syn::Error::new(meta.eq_token.span, "expected `(`")),
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

    pub fn as_name_value(&self) -> syn::Result<&MetaNameValue> {
        match self {
            Meta::NameValue(meta) => Ok(meta),
            Meta::Path(path) => Err(path.error(format!(
                "expected a value for this attribute: `{} = ...`",
                path.to_string(),
            ))),
            Meta::List(meta) => Err(syn::Error::new(
                meta.delimiter.span().open(),
                "expected `=`",
            )),
            Meta::NameList(meta) => Err(syn::Error::new(
                meta.list.delimiter.span().join(),
                "expected a valid expression",
            )),
            Meta::Expr(expr) => Err(expr.error("expected a path followed by `=`")),
            Meta::Verbatim(tokens) => Err(tokens.error("expected a path followed by `=`")),
        }
    }

    pub fn as_name_list(&self) -> syn::Result<&MetaNameList> {
        match self {
            Meta::NameList(meta) => Ok(meta),
            Meta::Path(path) => Err(path.error(format!(
                "expected a value for this attribute: `{} = ...(...)`",
                path.to_string(),
            ))),
            Meta::List(meta) => Err(syn::Error::new(
                meta.delimiter.span().open(),
                format!(
                    "expected a name for this list: `... = {}(...)`",
                    meta.path.to_string(),
                ),
            )),
            Meta::NameValue(meta) => Err(syn::Error::new(
                meta.value.span(),
                format!(
                    "expected a list as value: `{} = ...(...)`",
                    meta.path.to_string(),
                ),
            )),
            Meta::Expr(expr) => Err(expr.error("expected a named list: `... = ...(...)`")),
            Meta::Verbatim(tokens) => Err(tokens.error("expected a named list: `... = ...(...)`")),
        }
    }

    pub fn as_expr(&self) -> syn::Result<&Expr> {
        let span = match self {
            Meta::Expr(expr) => return Ok(expr),
            Meta::Path(path) => path.span(),
            Meta::List(meta) => meta.delimiter.span().join(),
            Meta::NameList(meta) => meta
                .path
                .span()
                .join(meta.list.delimiter.span().close())
                .unwrap_or(meta.path.span()),
            Meta::NameValue(meta) => meta
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
            Meta::NameList(meta) => meta
                .path
                .span()
                .join(meta.list.delimiter.span().close())
                .unwrap_or(meta.path.span()),
            Meta::NameValue(meta) => meta
                .path
                .span()
                .join(meta.value.span())
                .unwrap_or(meta.path.span()),
            Meta::Expr(expr) => expr.span(),
        };
        Err(syn::Error::new(span, "expected a custom token stream"))
    }

    pub fn value_as_meta(&self) -> syn::Result<Meta> {
        match self {
            Meta::NameList(meta) => Ok(Meta::List(meta.list.clone())),
            Meta::NameValue(meta) => match &meta.value {
                syn::Expr::Path(expr) => Ok(Meta::Path(expr.path.clone())),
                _ => Ok(Meta::Verbatim(meta.value.to_token_stream())),
            },
            _ => Err(self.error("expected a name-value or name-list attribute")),
        }
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
                    let eq_token = fork.parse::<Token![=]>()?;
                    match fork.call(parse_meta_list) {
                        Ok(list) => {
                            input.advance_to(&fork);
                            Ok(Self::NameList(MetaNameList {
                                path,
                                eq_token,
                                list,
                            }))
                        }
                        Err(_) => Ok(Self::NameValue(input.call(parse_meta_name_value)?)),
                    }
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

impl syn::parse::Parse for MetaNameList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.call(parse_meta_path)?,
            eq_token: input.parse()?,
            list: input.parse()?,
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

/// Custom implementation of `syn::parse::Parse` for `MetaNameValue`.
fn parse_meta_name_value(input: syn::parse::ParseStream) -> syn::Result<MetaNameValue> {
    Ok(MetaNameValue {
        path: input.call(parse_meta_path)?,
        eq_token: input.parse()?,
        value: input.parse()?,
    })
}

impl quote::ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Meta::Path(path) => path.to_tokens(tokens),
            Meta::List(list) => list.to_tokens(tokens),
            Meta::NameValue(name_value) => name_value.to_tokens(tokens),
            Meta::NameList(name_list) => name_list.to_tokens(tokens),
            Meta::Expr(expr) => expr.to_tokens(tokens),
            Meta::Verbatim(verbatim) => verbatim.to_tokens(tokens),
        }
    }
}

impl quote::ToTokens for MetaNameList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.list.to_tokens(tokens);
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
    fn parse_name_value() {
        let meta = meta! { foo = 42 };
        let Meta::NameValue(name_value) = meta else {
            panic!("expected Meta::NameValue");
        };
        assert!(name_value.path.is_strict("foo"));
        assert_eq!(name_value.value.as_literal_integer::<usize>().unwrap(), 42);
    }

    #[test]
    fn parse_name_list() {
        let meta = meta! { foo = bar(1, 2, 3) };
        let Meta::NameList(name_list) = meta else {
            panic!("expected Meta::NameList");
        };
        assert!(name_list.path.is_strict("foo"));
        assert!(name_list.list.path.is_strict("bar"));
        assert_eq!(name_list.list.tokens.to_string(), "1 , 2 , 3");
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
        assert_eq!(meta! { foo }.is_path_or_empty_list(), true);
        assert_eq!(meta! { some_node }.is_path_or_empty_list(), true);
        assert_eq!(meta! { foo() }.is_path_or_empty_list(), true);
        assert_eq!(meta! { some_node() }.is_path_or_empty_list(), true);
        assert_eq!(meta! { foo = 42 }.is_path_or_empty_list(), false);
        assert_eq!(meta! { foo = bar(1, 2, 3) }.is_path_or_empty_list(), false);
        assert_eq!(meta! { foo(answer = 42) }.is_path_or_empty_list(), false);
        assert_eq!(meta! { some_node(hello) }.is_path_or_empty_list(), false);
        assert_eq!(meta! { 42 }.is_path_or_empty_list(), false);
        assert_eq!(meta! { [verbatim] }.is_path_or_empty_list(), false);
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
