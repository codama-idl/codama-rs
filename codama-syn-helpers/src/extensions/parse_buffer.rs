use proc_macro2::{TokenStream, TokenTree};
use syn::{parse::ParseBuffer, Token};

pub trait ParseBufferExtension<'a> {
    fn get_self(&self) -> &ParseBuffer<'a>;

    /// Advance the buffer until we reach a comma or the end of the buffer.
    /// Returns the consumed tokens as a `TokenStream`.
    fn parse_arg(&self) -> syn::Result<TokenStream> {
        self.get_self().step(|cursor| {
            let mut tts = Vec::new();
            let mut rest = *cursor;
            while let Some((tt, next)) = rest.token_tree() {
                match &tt {
                    TokenTree::Punct(punct) if punct.as_char() == ',' => {
                        return Ok((tts.into_iter().collect(), rest));
                    }
                    _ => {
                        tts.push(tt);
                        rest = next
                    }
                }
            }
            Ok((tts.into_iter().collect(), rest))
        })
    }

    /// Fork the current buffer and move the original buffer to the end of the argument.
    fn fork_arg(&self) -> syn::Result<ParseBuffer<'a>> {
        let this = self.get_self();
        let fork = this.fork();
        this.parse_arg()?;
        Ok(fork)
    }

    // Check if the buffer is empty or the next token is a comma.
    fn is_end_of_arg(&self) -> bool {
        let this = self.get_self();
        this.is_empty() || this.peek(Token![,])
    }

    /// Check if the next token is an empty group.
    fn is_empty_group(&self) -> bool {
        match self.get_self().fork().parse::<proc_macro2::Group>() {
            Ok(group) => group.stream().is_empty(),
            Err(_) => false,
        }
    }
}

impl<'a> ParseBufferExtension<'a> for ParseBuffer<'a> {
    fn get_self(&self) -> &ParseBuffer<'a> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_buffer {
        ($ty:ty, $callback:expr) => {{
            struct TestBuffer($ty);
            impl syn::parse::Parse for TestBuffer {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    let result = ($callback)(input)?; // Call the provided callback
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
    fn parse_arg() {
        let test = test_buffer!(
            (String, String),
            |input: syn::parse::ParseStream| -> syn::Result<(String, String)> {
                let arg = input.parse_arg()?;
                Ok((arg.to_string(), input.to_string()))
            }
        );

        assert_eq!(
            test("foo , bar , baz").unwrap(),
            ("foo".into(), ", bar , baz".into())
        );
        assert_eq!(
            test(", bar , baz").unwrap(),
            ("".into(), ", bar , baz".into())
        );
        assert_eq!(
            test("(foo , bar), baz").unwrap(),
            ("(foo , bar)".into(), ", baz".into())
        );
        assert_eq!(
            test("foo bar baz").unwrap(),
            ("foo bar baz".into(), "".into())
        );
        assert_eq!(test("").unwrap(), ("".into(), "".into()));
    }

    #[test]
    fn fork_arg() {
        let test = test_buffer!(
            (String, String),
            |input: syn::parse::ParseStream| -> syn::Result<(String, String)> {
                let arg = input.fork_arg()?;
                Ok((arg.to_string(), input.to_string()))
            }
        );

        assert_eq!(
            test("foo , bar , baz").unwrap(),
            ("foo , bar , baz".into(), ", bar , baz".into())
        );
        assert_eq!(
            test("foo bar baz").unwrap(),
            ("foo bar baz".into(), "".into())
        );
    }

    #[test]
    fn is_end_of_arg() {
        let test = test_buffer!(
            bool,
            |input: syn::parse::ParseStream| -> syn::Result<bool> { Ok(input.is_end_of_arg()) }
        );

        assert_eq!(test("").unwrap(), true);
        assert_eq!(test(", bar").unwrap(), true);
        assert_eq!(test("foo").unwrap(), false);
    }

    #[test]
    fn is_empty_group() {
        let test = test_buffer!(
            bool,
            |input: syn::parse::ParseStream| -> syn::Result<bool> { Ok(input.is_empty_group()) }
        );

        assert_eq!(test("()").unwrap(), true);
        assert_eq!(test("[]").unwrap(), true);
        assert_eq!(test("{}").unwrap(), true);
        assert_eq!(test("").unwrap(), false);
        assert_eq!(test("foo").unwrap(), false);
        assert_eq!(test("(foo)").unwrap(), false);
    }
}
