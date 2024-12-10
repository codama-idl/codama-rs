use proc_macro2::{token_stream, Span};
use quote::TokenStreamExt;
use std::fmt::Display;
use syn::{
    parenthesized,
    parse::{ParseStream, Parser},
    Error, Path, Result, Token,
};

pub struct AttributeMeta<'a> {
    pub start: Span,
    pub input: ParseStream<'a>,
}

impl<'a> AttributeMeta<'a> {
    pub fn new(input: ParseStream<'a>) -> Self {
        Self {
            start: input.cursor().span(),
            input,
        }
    }

    pub fn parse_path(&self) -> Result<Path> {
        self.input.parse::<Path>()
    }

    pub fn parse_metas(&self, logic: impl FnMut(AttributeMeta) -> Result<()>) -> Result<()> {
        let content;
        parenthesized!(content in self.input);
        parse_metas_inside_parenthesis(&content, logic)
    }

    pub fn error(&self, msg: impl Display) -> Error {
        let start = self.start;
        let end = self.input.cursor().span();

        let mut token_stream = token_stream::TokenStream::new();
        token_stream.append(proc_macro2::Ident::new("start", start));
        token_stream.append(proc_macro2::Ident::new("end", end));
        Error::new_spanned(token_stream, msg)
    }

    pub fn parser(logic: impl FnMut(AttributeMeta) -> Result<()>) -> impl Parser<Output = ()> {
        |input: ParseStream| {
            if input.is_empty() {
                Ok(())
            } else {
                parse_metas_inside_parenthesis(input, logic)
            }
        }
    }
}

fn parse_metas_inside_parenthesis(
    input: ParseStream,
    mut logic: impl FnMut(AttributeMeta) -> Result<()>,
) -> Result<()> {
    loop {
        logic(AttributeMeta::new(input))?;
        if input.is_empty() {
            return Ok(());
        }
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use crate::syn_traits::ParseBuffer;

    use super::*;

    macro_rules! test_attribute_meta {
        ($ty:ty, $callback:expr) => {{
            struct TestBuffer($ty);
            impl syn::parse::Parse for TestBuffer {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    let meta = AttributeMeta::new(input);
                    let result = ($callback)(meta)?; // Call the provided callback
                    input.parse::<proc_macro2::TokenStream>()?; // Consume the rest of the input
                    Ok(Self(result))
                }
            }
            move |input: &str| -> syn::Result<$ty> {
                syn::parse_str::<TestBuffer>(input).map(|x| x.0)
            }
        }};
    }

    #[test]
    fn parse_path() {
        let test = test_attribute_meta!(
            (String, String),
            |meta: AttributeMeta| -> syn::Result<(String, String)> {
                let path = meta.parse_path()?;
                Ok((path.to_token_stream().to_string(), meta.input.to_string()))
            }
        );

        assert_eq!(
            test("foo , bar , baz").unwrap(),
            ("foo".into(), ", bar , baz".into())
        );
    }

    #[test]
    fn parse_metas() {
        let test = test_attribute_meta!(
            Vec<String>,
            |meta: AttributeMeta| -> syn::Result<Vec<String>> {
                let mut metas = Vec::new();
                meta.parse_metas(|m| {
                    metas.push(m.input.to_string());
                    m.input.consume_arg()?;
                    Ok(())
                })?;
                Ok(metas)
            }
        );

        assert_eq!(
            test("(bar , baz)").unwrap(),
            vec!["bar , baz".to_string(), "baz".to_string()]
        );
    }
}
