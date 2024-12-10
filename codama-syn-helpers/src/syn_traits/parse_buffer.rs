use proc_macro2::TokenTree;
use syn::Token;

pub trait ParseBuffer<'a> {
    fn get_self(&self) -> &syn::parse::ParseBuffer<'a>;

    /// Advance the buffer until we reach a comma or the end of the buffer.
    fn consume_arg(&self) -> syn::Result<()> {
        self.get_self().step(|cursor| {
            let mut rest = *cursor;
            while let Some((tt, next)) = rest.token_tree() {
                match &tt {
                    TokenTree::Punct(punct) if punct.as_char() == ',' => {
                        return Ok(((), rest));
                    }
                    _ => rest = next,
                }
            }
            Ok(((), rest))
        })
    }

    /// Fork the current buffer and move the original buffer to the end of the argument.
    fn fork_arg(&self) -> syn::Result<syn::parse::ParseBuffer<'a>> {
        let this = self.get_self();
        let fork = this.fork();
        this.consume_arg()?;
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

impl<'a> ParseBuffer<'a> for syn::parse::ParseBuffer<'a> {
    fn get_self(&self) -> &syn::parse::ParseBuffer<'a> {
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
    fn consume_arg() {
        let test = test_buffer!(
            String,
            |input: syn::parse::ParseStream| -> syn::Result<String> {
                input.consume_arg()?;
                Ok(input.to_string())
            }
        );

        assert_eq!(test("foo , bar , baz").unwrap(), ", bar , baz");
        assert_eq!(test(", bar , baz").unwrap(), ", bar , baz");
        assert_eq!(test("(foo , bar), baz").unwrap(), ", baz");
        assert!(test("foo bar baz").unwrap().is_empty());
        assert!(test("").unwrap().is_empty());
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
            ("foo , bar , baz".to_string(), ", bar , baz".to_string())
        );
        assert_eq!(
            test("foo bar baz").unwrap(),
            ("foo bar baz".to_string(), "".to_string())
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