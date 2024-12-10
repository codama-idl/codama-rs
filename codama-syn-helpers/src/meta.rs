use proc_macro2::TokenStream;
use syn::{
    parse::discouraged::Speculative,
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

#[cfg(test)]
mod tests {
    use crate::syn_traits::{Expr, Path};

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
