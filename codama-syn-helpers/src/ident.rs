/// Converts a string from any common casing (camelCase, PascalCase,
/// kebab-case, snake_case) to snake_case.
///
/// Word boundaries match the conventions a Rust renderer wants: an uppercase
/// run splits before its last letter when a lowercase letter follows, so
/// `"HTTPServer"` becomes `"http_server"` and `"tokenAMint"` becomes
/// `"token_a_mint"`. A digit ends a word only when an uppercase letter follows
/// a lowercase run, so `"seed2Bump"` becomes `"seed2_bump"` while `"mint_2"`
/// and `"this123"` are left alone.
///
/// Characters that are not alphanumeric are treated as separators and dropped,
/// so the result can be empty (`"!!!"` gives `""`) and can start with a digit.
/// Callers that need a valid Rust identifier must handle both.
pub fn to_snake_case(input: &str) -> String {
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

/// The case of the last cased character seen in the current word. A digit
/// carries the mode of whatever preceded it, so "seed2Bump" splits after the
/// digit while "1X" does not.
#[derive(Clone, Copy, PartialEq)]
enum WordMode {
    Boundary,
    Lowercase,
    Uppercase,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_camel_case() {
        assert_eq!(to_snake_case("tokenProgram"), "token_program");
        assert_eq!(to_snake_case("myValue"), "my_value");
        assert_eq!(to_snake_case("unwrappedMint"), "unwrapped_mint");
    }

    #[test]
    fn from_pascal_case() {
        assert_eq!(
            to_snake_case("HelloThisIs7PascalCaseWords"),
            "hello_this_is7_pascal_case_words"
        );
    }

    #[test]
    fn from_kebab_case() {
        assert_eq!(to_snake_case("order-id"), "order_id");
    }

    #[test]
    fn from_snake_case() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
        assert_eq!(
            to_snake_case("hello_this_is__a_snake_case"),
            "hello_this_is_a_snake_case"
        );
    }

    #[test]
    fn from_title_case() {
        assert_eq!(
            to_snake_case("Hello This is a Long Title!"),
            "hello_this_is_a_long_title"
        );
    }

    #[test]
    fn splits_uppercase_runs() {
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
        assert_eq!(to_snake_case("parseJSONData"), "parse_json_data");
        assert_eq!(to_snake_case("SPLToken"), "spl_token");
        assert_eq!(to_snake_case("tokenAMint"), "token_a_mint");
        assert_eq!(to_snake_case("poolNFTMint"), "pool_nft_mint");
        assert_eq!(to_snake_case("someID"), "some_id");
        // An uppercase run with no lowercase letter after it stays whole.
        assert_eq!(to_snake_case("SCREAMING_SNAKE"), "screaming_snake");
    }

    #[test]
    fn keeps_digit_runs_with_their_word() {
        assert_eq!(to_snake_case("this123"), "this123");
        assert_eq!(to_snake_case("This123 str1ng"), "this123_str1ng");
        assert_eq!(to_snake_case("mint_2"), "mint_2");
        assert_eq!(to_snake_case("token_0"), "token_0");
        // A digit ends a word only when an uppercase letter follows it and the
        // run started from lowercase.
        assert_eq!(to_snake_case("seed2Bump"), "seed2_bump");
        assert_eq!(to_snake_case("2Mint"), "2mint");
        assert_eq!(to_snake_case("a_2B"), "a_2b");
        assert_eq!(to_snake_case("mint_2Bump"), "mint_2bump");
    }

    #[test]
    fn strips_non_alphanumeric() {
        assert_eq!(
            to_snake_case("crate::hello:world?*,this+is!a#test"),
            "crate_hello_world_this_is_a_test"
        );
        assert_eq!(to_snake_case("!!!"), "");
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn is_idempotent_over_ascii() {
        for input in [
            "tokenProgram",
            "HTTPServer",
            "tokenAMint",
            "seed2Bump",
            "mint_2",
            "order-id",
            "SCREAMING_SNAKE",
            "2Mint",
            "a_2B",
            "!!!",
        ] {
            let once = to_snake_case(input);
            assert_eq!(to_snake_case(&once), once, "not idempotent for {input:?}");
        }
    }
}
