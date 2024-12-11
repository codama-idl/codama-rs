use crate::syn_traits::{Path as _, ToTokens as _};
use proc_macro2::TokenStream;
use syn::{
    parse::discouraged::Speculative,
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
    MetaList, MetaNameValue, Path, Token,
};

#[derive(Debug)]
pub enum Meta {
    Path(Path),
    List(MetaList),
    NameValue(MetaNameValue),
    NameList(MetaNameList),
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
            Meta::Verbatim(tokens) => Err(tokens.error("expected a path")),
        }
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
            Meta::Verbatim(tokens) => Err(tokens.error("expected a named list: `... = ...(...)`")),
        }
    }

    pub fn as_verbatim(&self, msg: impl std::fmt::Display) -> syn::Result<&TokenStream> {
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
        };
        Err(syn::Error::new(span, msg))
    }
}

impl syn::parse::Parse for Meta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        match fork.parse::<Path>() {
            Ok(path) => {
                if fork.peek(Paren) || fork.peek(Bracket) || fork.peek(Brace) {
                    Ok(Self::List(input.parse()?))
                } else if fork.peek(Token![=]) {
                    let eq_token = fork.parse::<Token![=]>()?;
                    match fork.parse::<MetaList>() {
                        Ok(list) => {
                            input.advance_to(&fork);
                            Ok(Self::NameList(MetaNameList {
                                path,
                                eq_token,
                                list,
                            }))
                        }
                        Err(_) => Ok(Self::NameValue(input.parse()?)),
                    }
                } else {
                    input.advance_to(&fork);
                    Ok(Self::Path(path))
                }
            }
            Err(_) => Ok(Self::Verbatim(input.parse()?)),
        }
    }
}

impl syn::parse::Parse for MetaNameList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            eq_token: input.parse()?,
            list: input.parse()?,
        })
    }
}

impl quote::ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Meta::Path(path) => path.to_tokens(tokens),
            Meta::List(list) => list.to_tokens(tokens),
            Meta::NameValue(name_value) => name_value.to_tokens(tokens),
            Meta::NameList(name_list) => name_list.to_tokens(tokens),
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
    use crate::syn_traits::{Expr as _, Path as _};

    use super::*;

    #[test]
    fn path() {
        let meta = syn::parse_str::<Meta>("foo");
        let Ok(Meta::Path(path)) = meta else {
            panic!("expected Meta::Path");
        };
        assert!(path.is_strict("foo"));
    }

    #[test]
    fn list() {
        let meta = syn::parse_str::<Meta>("foo(1, 2, 3)");
        let Ok(Meta::List(list)) = meta else {
            panic!("expected Meta::List");
        };
        assert!(list.path.is_strict("foo"));
        assert_eq!(list.tokens.to_string(), "1 , 2 , 3");
    }

    #[test]
    fn name_value() {
        let meta = syn::parse_str::<Meta>("foo = 42");
        let Ok(Meta::NameValue(name_value)) = meta else {
            panic!("expected Meta::NameValue");
        };
        assert!(name_value.path.is_strict("foo"));
        assert_eq!(name_value.value.as_literal_integer::<usize>().unwrap(), 42);
    }

    #[test]
    fn name_list() {
        let meta = syn::parse_str::<Meta>("foo = bar(1, 2, 3)");
        let Ok(Meta::NameList(name_list)) = meta else {
            panic!("expected Meta::NameList");
        };
        assert!(name_list.path.is_strict("foo"));
        assert!(name_list.list.path.is_strict("bar"));
        assert_eq!(name_list.list.tokens.to_string(), "1 , 2 , 3");
    }

    #[test]
    fn verbatim() {
        let meta = syn::parse_str::<Meta>("[==> 42 <==]");
        let Ok(Meta::Verbatim(verbatim)) = meta else {
            panic!("expected Meta::Verbatim");
        };
        assert_eq!(verbatim.to_string(), "[==> 42 <==]");
    }
}
