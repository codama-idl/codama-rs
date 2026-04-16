use heck::ToSnakeCase;
use proc_macro2::Span;

/// Converts a string from any common casing (camelCase, kebab-case, PascalCase)
/// to snake_case.
pub fn to_snake_case(input: &str) -> String {
    input.to_snake_case()
}

/// Converts a string to a valid snake_case `syn::Ident`, handling Rust
/// keywords and leading digits.
pub fn snake_case_ident(input: &str) -> syn::Ident {
    let snake = to_snake_case(input);
    string_to_ident(&snake)
}

/// Converts a string to a valid `syn::Ident`, handling Rust keywords
/// (e.g. `type` -> `r#type`) and leading digits (e.g. `3foo` -> `_3foo`).
/// Panics if the input is empty.
pub fn string_to_ident(input: &str) -> syn::Ident {
    if input.is_empty() {
        panic!("cannot create an identifier from an empty string");
    }

    let mut ident = input.to_string();
    if ident
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        ident.insert(0, '_');
    }

    if syn::parse_str::<syn::Ident>(&ident).is_err() {
        if matches!(ident.as_str(), "self" | "super" | "crate") {
            syn::Ident::new(&format!("{ident}_"), Span::call_site())
        } else {
            syn::Ident::new_raw(&ident, Span::call_site())
        }
    } else {
        syn::Ident::new(&ident, Span::call_site())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_snake_case_from_camel_case() {
        assert_eq!(to_snake_case("tokenProgram"), "token_program");
        assert_eq!(to_snake_case("myValue"), "my_value");
    }

    #[test]
    fn to_snake_case_from_kebab_case() {
        assert_eq!(to_snake_case("order-id"), "order_id");
    }

    #[test]
    fn to_snake_case_from_snake_case() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    fn to_snake_case_strips_non_alphanumeric() {
        assert_eq!(to_snake_case("!!!"), "");
    }

    #[test]
    fn snake_case_ident_basic() {
        assert_eq!(
            snake_case_ident("tokenProgram").to_string(),
            "token_program"
        );
        assert_eq!(snake_case_ident("order-id").to_string(), "order_id");
    }

    #[test]
    fn string_to_ident_handles_keywords() {
        assert_eq!(string_to_ident("type").to_string(), "r#type");
        assert_eq!(string_to_ident("self").to_string(), "self_");
        assert_eq!(string_to_ident("super").to_string(), "super_");
        assert_eq!(string_to_ident("crate").to_string(), "crate_");
    }

    #[test]
    fn string_to_ident_handles_leading_digit() {
        assert_eq!(string_to_ident("3foo").to_string(), "_3foo");
    }

    #[test]
    #[should_panic(expected = "cannot create an identifier from an empty string")]
    fn string_to_ident_panics_on_empty() {
        string_to_ident("");
    }
}
