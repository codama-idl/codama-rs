use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// A string normalized to `snake_case`.
///
/// The counterpart to `CamelCaseString`. Word boundaries follow the usual
/// conventions rather than `CamelCaseString`'s: an uppercase run is split
/// before its last letter when a lowercase letter follows, so `"HTTPServer"`
/// becomes `"http_server"` and `"tokenAMint"` becomes `"token_a_mint"`.
///
/// Convert from the original string rather than from a `CamelCaseString`.
/// `CamelCaseString` folds an uppercase run into one word and drops the
/// separator before a digit, so `"tokenAMint"` is already `"tokenAmint"` and
/// `"mint_2"` is already `"mint2"` by then, and those boundaries cannot be
/// recovered.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SnakeCaseString(String);

impl SnakeCaseString {
    pub fn new<T>(string: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(to_snake_case(string.as_ref()))
    }
}

impl From<SnakeCaseString> for String {
    fn from(val: SnakeCaseString) -> Self {
        val.0
    }
}

impl From<String> for SnakeCaseString {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl From<&str> for SnakeCaseString {
    fn from(string: &str) -> Self {
        Self::new(string)
    }
}

impl Deref for SnakeCaseString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for SnakeCaseString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// The case of the last cased character seen in the current word. A digit
/// carries the mode of whatever preceded it, so "seed2Bump" splits after the
/// digit while "1X" does not.
#[derive(Clone, Copy, PartialEq)]
enum WordMode {
    Boundary,
    Lowercase,
    Uppercase,
}

fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut new_word = true;
    let mut mode = WordMode::Boundary;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        if !c.is_alphanumeric() {
            new_word = true;
            mode = WordMode::Boundary;
            i += 1;
            continue;
        }

        if c.is_uppercase() {
            let next_is_lowercase = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            // A lowercase run ending in an uppercase letter starts a new word
            // ("camelCase"), and so does the last letter of an uppercase run
            // when a lowercase letter follows it ("HTTPServer").
            if mode == WordMode::Lowercase || (mode == WordMode::Uppercase && next_is_lowercase) {
                new_word = true;
            }
        }

        if new_word && !result.is_empty() {
            result.push('_');
        }
        result.extend(c.to_lowercase());
        new_word = false;

        if c.is_lowercase() {
            mode = WordMode::Lowercase;
        } else if c.is_uppercase() {
            mode = WordMode::Uppercase;
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_title_case() {
        let value = SnakeCaseString::new(String::from("Hello This is a Long Title!"));
        assert_eq!(value.0, "hello_this_is_a_long_title");
    }

    #[test]
    fn parse_from_numbers() {
        let value = SnakeCaseString::new(String::from("This123 str1ng has 456n numbers"));
        assert_eq!(value.0, "this123_str1ng_has_456n_numbers");
    }

    #[test]
    fn parse_from_snake_case() {
        let value = SnakeCaseString::new(String::from("hello_this_is__a_snake_case"));
        assert_eq!(value.0, "hello_this_is_a_snake_case");
    }

    #[test]
    fn parse_from_pascal_case() {
        let value = SnakeCaseString::new(String::from("HelloThisIs7PascalCaseWords"));
        assert_eq!(value.0, "hello_this_is7_pascal_case_words");
    }

    #[test]
    fn parse_from_camel_case() {
        let value = SnakeCaseString::new(String::from("unwrappedMint"));
        assert_eq!(value.0, "unwrapped_mint");
    }

    #[test]
    fn parse_from_kebab_case() {
        let value = SnakeCaseString::new(String::from("order-id"));
        assert_eq!(value.0, "order_id");
    }

    #[test]
    fn parse_from_special_chars() {
        let value = SnakeCaseString::new(String::from("crate::hello:world?*,this+is!a#test"));
        assert_eq!(value.0, "crate_hello_world_this_is_a_test");
    }

    #[test]
    fn parse_from_uppercase_run() {
        for (input, expected) in [
            ("HTTPServer", "http_server"),
            ("parseJSONData", "parse_json_data"),
            ("SPLToken", "spl_token"),
            ("tokenAMint", "token_a_mint"),
            // An uppercase run with no lowercase letter after it stays whole.
            ("SCREAMING_SNAKE", "screaming_snake"),
            ("someID", "some_id"),
        ] {
            assert_eq!(SnakeCaseString::new(input).0, expected);
        }
    }

    #[test]
    fn parse_from_non_alphanumeric() {
        let value = SnakeCaseString::new("!!!");
        assert_eq!(value.0, "");
    }

    #[test]
    fn parse_from_digits() {
        for (input, expected) in [
            // A digit run stays with the word it belongs to.
            ("this123", "this123"),
            ("This123 str1ng", "this123_str1ng"),
            // An existing separator before a digit survives.
            ("mint_2", "mint_2"),
            ("seed_1", "seed_1"),
            ("token_0", "token_0"),
            // A digit ends a word only when an uppercase letter follows.
            ("seed2Bump", "seed2_bump"),
            // A leading digit run does not split before an uppercase letter,
            // and neither does one that starts a word after a separator.
            ("2Mint", "2mint"),
            ("a_2B", "a_2b"),
            ("mint_2Bump", "mint_2bump"),
        ] {
            assert_eq!(SnakeCaseString::new(input).0, expected);
        }
    }

    #[test]
    fn parse_from_pda_seed_names() {
        for (input, expected) in [
            ("tokenProgram", "token_program"),
            ("already_snake", "already_snake"),
            ("tokenBMint", "token_b_mint"),
            ("poolNFTMint", "pool_nft_mint"),
            ("feeUSDAccount", "fee_usd_account"),
        ] {
            assert_eq!(SnakeCaseString::new(input).0, expected);
        }
    }

    #[test]
    fn double_parse() {
        let value = to_snake_case("myValue");
        let value = to_snake_case(&value);
        assert_eq!(value, "my_value");
    }

    #[test]
    fn new_from_string() {
        let value = SnakeCaseString::new(String::from("my_value"));
        assert_eq!(value.0, "my_value");
    }

    #[test]
    fn new_from_str() {
        let value = SnakeCaseString::new("my_value");
        assert_eq!(value.0, "my_value");
    }

    #[test]
    fn new_from_self() {
        let value = SnakeCaseString::new(SnakeCaseString::new("myValue"));
        assert_eq!(value.0, "my_value");
    }

    #[test]
    fn from_string() {
        let value: SnakeCaseString = String::from("my_value").into();
        assert_eq!(value.0, "my_value");
    }

    #[test]
    fn from_str() {
        let value: SnakeCaseString = "my_value".into();
        assert_eq!(value.0, "my_value");
    }

    #[test]
    fn into_string() {
        let value: String = SnakeCaseString::new("my_value").into();
        assert_eq!(value, "my_value");
    }

    #[test]
    fn deref() {
        let value = SnakeCaseString::new("Hello World!");
        assert_eq!(*value, "hello_world");
    }

    #[test]
    fn as_ref() {
        let value = SnakeCaseString::new("Hello World!");
        assert_eq!(value.as_ref(), "hello_world");
    }

    #[test]
    fn to_json() {
        let value = SnakeCaseString::new("helloWorld");
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"hello_world\"");
    }

    #[test]
    fn from_json() {
        let json = "\"hello_world\"";
        let value: SnakeCaseString = serde_json::from_str(json).unwrap();
        assert_eq!(value, SnakeCaseString::new("hello_world"));
    }
}
