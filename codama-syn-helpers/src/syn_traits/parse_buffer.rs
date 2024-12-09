use syn::Token;

pub trait ParseBuffer<'a> {
    fn get_self(&self) -> &syn::parse::ParseBuffer<'a>;

    /// Advance the buffer until we reach a comma or the end of the buffer.
    fn parse_end_of_arg(&self) -> syn::Result<()> {
        let this = self.get_self();
        while !this.is_empty() && !this.peek(Token![,]) {
            this.parse::<proc_macro2::TokenTree>()?;
        }
        Ok(())
    }

    // Check if the buffer is empty or the next token is a comma.
    fn is_end_of_arg(&self) -> bool {
        let this = self.get_self();
        this.is_empty() || this.peek(Token![,])
    }

    /// Check if the next token is an empty group.
    fn is_empty_group(&self) -> bool {
        match self.get_self().parse::<proc_macro2::Group>() {
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
    fn parse_end_of_arg() {
        let test = test_buffer!(
            String,
            |input: syn::parse::ParseStream| -> syn::Result<String> {
                input.parse_end_of_arg()?;
                Ok(input.to_string())
            }
        );

        assert_eq!(test("foo , bar , baz").unwrap(), ", bar , baz");
        assert_eq!(test("(foo , bar), baz").unwrap(), ", baz");
        assert!(test("foo bar baz").unwrap().is_empty());
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
