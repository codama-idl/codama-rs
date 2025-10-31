use crate::extensions::*;
use derive_more::From;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::fmt::Display;
use syn::{
    ext::IdentExt,
    parse::discouraged::Speculative,
    token::{Brace, Bracket, Paren},
    Expr, MacroDelimiter, MetaList, Path, Token,
};

#[derive(Debug, From)]
pub enum Meta {
    /// A path followed by an equal sign and a single Meta — e.g. `my_attribute = my_value`.
    PathValue(PathValue),
    /// A path followed by a wrapped list of Metas — e.g. `my_attribute(one, two, three)`.
    /// This accepts an optional equal sign before the list — e.g. `my_attribute = (one, two, three)`.
    PathList(PathList),
    /// An expression — e.g. `my_attribute`, `42`, `"hello"` or `a + b`.
    Expr(Expr),
    /// A verbatim token stream — e.g. `[<=>]`.
    /// This is the fallback for any other Meta that does not match the other variants.
    Verbatim(TokenStream),
}

#[derive(Debug)]
pub struct PathValue {
    pub path: Path,
    pub eq_token: Token![=],
    pub value: Box<Meta>,
}

#[derive(Debug)]
pub struct PathList {
    pub path: Path,
    pub eq_token: Option<Token![=]>,
    pub delimiter: MacroDelimiter,
    pub tokens: TokenStream,
}

impl Meta {
    pub fn path(&self) -> syn::Result<&Path> {
        match self {
            Meta::PathList(pl) => Ok(&pl.path),
            Meta::PathValue(pv) => Ok(&pv.path),
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

    pub fn is_path_or_list(&self) -> bool {
        matches!(self, Meta::PathList(_) | Meta::Expr(Expr::Path(_)))
    }

    pub fn is_path_or_empty_list(&self) -> bool {
        match self {
            Meta::PathList(pl) if pl.tokens.is_empty() => true,
            Meta::Expr(Expr::Path(_)) => true,
            _ => false,
        }
    }

    pub fn as_path(&self) -> syn::Result<&Path> {
        match self {
            Meta::Expr(Expr::Path(expr)) => Ok(&expr.path),
            Meta::PathList(pl) => Err(pl
                .tokens_after_path()
                .error("unexpected tokens, expected a single path")),
            Meta::PathValue(pv) => Err(pv
                .tokens_after_path()
                .error("unexpected tokens, expected a single path")),
            meta => Err(meta.error("expected a path")),
        }
    }

    pub fn as_path_list(&self) -> syn::Result<&PathList> {
        match self {
            Meta::PathList(pl) => Ok(pl),
            Meta::PathValue(pv) => Err(pv.value.error(format!(
                "expected a list: `{} = (...)`",
                pv.path.to_string()
            ))),
            Meta::Expr(Expr::Path(expr)) => Err(expr.error(format!(
                "expected a list: `{0}(...)` or `{0} = (...)`",
                expr.path.to_string()
            ))),
            meta => Err(meta
                .error("expected a path followed by a list: `my_path(...)` or `my_path = (...)`")),
        }
    }

    pub fn as_path_value(&self) -> syn::Result<&PathValue> {
        match self {
            Meta::PathValue(pv) => Ok(pv),
            Meta::PathList(pl) => match pl.eq_token {
                Some(_) => Err(pl.tokens.error("expected a single value, found a list")),
                None => Err(pl.tokens.error(format!(
                    "expected `=` followed by a single value: `{} = ...`",
                    pl.path.to_string(),
                ))),
            },
            Meta::Expr(Expr::Path(expr)) => Err(expr.error(format!(
                "expected a value for that path: `{} = ...`",
                expr.path.to_string()
            ))),
            meta => Err(meta.error("expected a path followed by a value: `my_path = ...`")),
        }
    }

    pub fn as_expr(&self) -> syn::Result<&Expr> {
        match self {
            Meta::Expr(expr) => Ok(expr),
            meta => Err(meta.error("expected a valid expression")),
        }
    }

    pub fn as_verbatim(&self, msg: impl Display) -> syn::Result<&TokenStream> {
        match self {
            Meta::Verbatim(tokens) => Ok(tokens),
            meta => Err(meta.error(msg)),
        }
    }

    pub fn assert_directive(&self, directive: &str) -> syn::Result<&Self> {
        let path = self.path()?;
        if !path.is_strict(directive) {
            return Err(path.error(format!("expected #[codama({directive})] attribute")));
        };
        Ok(self)
    }
}

impl PathValue {
    /// Get the equal sign and value tokens.
    pub fn tokens_after_path(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.eq_token.to_tokens(&mut tokens);
        self.value.to_tokens(&mut tokens);
        tokens
    }
}

impl PathList {
    /// Get all tokens after the path, including the equal sign if present.
    pub fn tokens_after_path(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.eq_token.to_tokens(&mut tokens);
        delimiters_to_tokens(&self.delimiter, &self.tokens, &mut tokens);
        tokens
    }

    /// Get an equivalent `MetaList` from the path list.
    pub fn as_meta_list(&self) -> MetaList {
        MetaList {
            path: self.path.clone(),
            delimiter: self.delimiter.clone(),
            tokens: self.tokens.clone(),
        }
    }

    /// Iterate over all metas in the list.
    pub fn each(&self, logic: impl FnMut(Meta) -> syn::Result<()>) -> syn::Result<()> {
        self.as_meta_list().each(logic)
    }

    /// Parse all metas in the list.
    pub fn parse_metas(&self) -> syn::Result<Vec<Meta>> {
        self.as_meta_list().parse_metas()
    }

    /// Parse all arguments as comma-separated types.
    pub fn parse_comma_args<T: syn::parse::Parse>(&self) -> syn::Result<Vec<T>> {
        self.as_meta_list().parse_comma_args()
    }

    pub fn as_expr_array(&self) -> syn::Result<syn::ExprArray> {
        let syn::MacroDelimiter::Bracket(bracket_token) = self.delimiter else {
            return Err(self.error("expected an array delimited with `[]`"));
        };

        Ok(syn::ExprArray {
            attrs: vec![],
            bracket_token,
            elems: self.as_meta_list().parse_args_with(
                syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated,
            )?,
        })
    }
}

impl syn::parse::Parse for Meta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        match fork.call(parse_meta_path) {
            Ok(path) => {
                if fork.peek(Paren)
                    || fork.peek(Bracket)
                    || fork.peek(Brace)
                    || (fork.peek(Token![=])
                        && (fork.peek2(Paren) || fork.peek2(Bracket) || fork.peek2(Brace)))
                {
                    Ok(Self::PathList(input.parse()?))
                } else if fork.peek(Token![=]) {
                    Ok(Self::PathValue(input.parse()?))
                } else {
                    input.advance_to(&fork);
                    Ok(Self::Expr(syn::Expr::Path(syn::ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path,
                    })))
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

impl syn::parse::Parse for PathValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.call(parse_meta_path)?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl syn::parse::Parse for PathList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.call(parse_meta_path)?;
        let eq_token = input.parse()?;
        let (delimiter, tokens) = input.call(parse_delimiters)?;
        Ok(Self {
            path,
            eq_token,
            delimiter,
            tokens,
        })
    }
}

/// Parse a path without segment arguments and allowing any reserved keyword
/// except `true` and `false` on the first segment.
fn parse_meta_path(input: syn::parse::ParseStream) -> syn::Result<Path> {
    let fork = input.fork();
    let ident = syn::Ident::parse_any(&fork)?;
    if ident == "true" || ident == "false" {
        return Err(ident.error("unexpected reserved keyword"));
    }
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

/// Parses a custom token stream inside delimiters.
fn parse_delimiters(input: syn::parse::ParseStream) -> syn::Result<(MacroDelimiter, TokenStream)> {
    input.step(|cursor| match cursor.token_tree() {
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
    })
}

fn delimiters_to_tokens(delimiter: &MacroDelimiter, inner: &TokenStream, tokens: &mut TokenStream) {
    let (delim, span) = match delimiter {
        MacroDelimiter::Paren(paren) => (Delimiter::Parenthesis, paren.span),
        MacroDelimiter::Brace(brace) => (Delimiter::Brace, brace.span),
        MacroDelimiter::Bracket(bracket) => (Delimiter::Bracket, bracket.span),
    };
    let mut group = Group::new(delim, inner.clone());
    group.set_span(span.join());
    tokens.append(group);
}

impl ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Meta::PathList(m) => m.to_tokens(tokens),
            Meta::PathValue(m) => m.to_tokens(tokens),
            Meta::Expr(m) => m.to_tokens(tokens),
            Meta::Verbatim(m) => m.to_tokens(tokens),
        }
    }
}

impl ToTokens for PathValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl ToTokens for PathList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        delimiters_to_tokens(&self.delimiter, &self.tokens, tokens);
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
        let Meta::Expr(syn::Expr::Path(syn::ExprPath { path, .. })) = meta else {
            panic!("expected Meta::Path");
        };
        assert!(path.is_strict("foo"));
    }

    #[test]
    fn parse_path_list() {
        let meta = meta! { foo(1, 2, 3) };
        let Meta::PathList(meta) = meta else {
            panic!("expected Meta::List");
        };
        assert!(meta.path.is_strict("foo"));
        assert!(meta.eq_token.is_none());
        assert_eq!(meta.tokens.to_string(), "1 , 2 , 3");
    }

    #[test]
    fn parse_path_list_with_equal_sign() {
        let meta = meta! { foo = [1, 2, 3] };
        let Meta::PathList(meta) = meta else {
            panic!("expected Meta::List");
        };
        assert!(meta.path.is_strict("foo"));
        assert!(meta.eq_token.is_some());
        assert_eq!(meta.tokens.to_string(), "1 , 2 , 3");
    }

    #[test]
    fn parse_path_value_with_expression() {
        let meta: Meta = meta! { foo = 42 };
        let Meta::PathValue(meta) = meta else {
            panic!("expected Meta::PathValue");
        };
        assert!(meta.path.is_strict("foo"));
        let expr = meta.value.as_expr().unwrap();
        assert_eq!(expr.as_unsigned_integer::<usize>().unwrap(), 42);
    }

    #[test]
    fn parse_path_value_with_boolean() {
        let meta: Meta = meta! { foo = true };
        let Meta::PathValue(meta) = meta else {
            panic!("expected Meta::PathValue");
        };
        assert!(meta.path.is_strict("foo"));
        let expr = meta.value.as_expr().unwrap();
        assert!(expr.as_bool().unwrap());
    }

    #[test]
    fn parse_path_value_with_verbatim() {
        let meta = meta! { foo = ?what? };
        let Meta::PathValue(meta) = meta else {
            panic!("expected Meta::PathValue");
        };
        assert!(meta.path.is_strict("foo"));
        let value = meta.value.as_verbatim("expected verbatim").unwrap();
        assert_eq!(value.to_string(), "? what ?");
    }

    #[test]
    fn parse_path_value_with_list() {
        let meta = meta! { foo = bar(1, 2, 3) };
        let Meta::PathValue(meta) = meta else {
            panic!("expected Meta::PathValue");
        };
        assert!(meta.path.is_strict("foo"));
        let value = meta.value.as_path_list().unwrap();
        assert!(value.path.is_strict("bar"));
        assert_eq!(value.tokens.to_string(), "1 , 2 , 3");
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
        let list = meta.as_path_list()?;
        assert_eq!(list.path.to_string(), "foo");
        let metas = list.parse_metas()?;
        assert_eq!(metas.len(), 2);
        assert_eq!(
            metas[0].as_verbatim("expected [==> n <==]")?.to_string(),
            "[==> 1 <==]"
        );
        assert_eq!(
            metas[1].as_verbatim("expected [==> n <==]")?.to_string(),
            "[==> 2 <==]"
        );
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

        assert_eq!(
            meta! { foo(42) }.as_path().unwrap_err().to_string(),
            "unexpected tokens, expected a single path"
        );
        assert_eq!(
            meta! { foo = 42 }.as_path().unwrap_err().to_string(),
            "unexpected tokens, expected a single path"
        );
        assert_eq!(
            meta! { foo = bar(42) }.as_path().unwrap_err().to_string(),
            "unexpected tokens, expected a single path"
        );
        assert_eq!(
            meta! { [verbatim] }.as_path().unwrap_err().to_string(),
            "expected a path"
        );
    }

    #[test]
    fn as_path_list() {
        let meta = meta! { foo(1, 2, 3) };
        let list = meta.as_path_list().unwrap();
        assert!(list.path.is_strict("foo"));
        assert_eq!(list.tokens.to_string(), "1 , 2 , 3");

        assert_eq!(
            meta! { foo }.as_path_list().unwrap_err().to_string(),
            "expected a list: `foo(...)` or `foo = (...)`"
        );
        assert_eq!(
            meta! { foo = 42 }.as_path_list().unwrap_err().to_string(),
            "expected a list: `foo = (...)`"
        );
        assert_eq!(
            meta! { foo = bar(42) }
                .as_path_list()
                .unwrap_err()
                .to_string(),
            "expected a list: `foo = (...)`"
        );
        assert_eq!(
            meta! { [verbatim] }.as_path_list().unwrap_err().to_string(),
            "expected a path followed by a list: `my_path(...)` or `my_path = (...)`"
        );
    }
}
