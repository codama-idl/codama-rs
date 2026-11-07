//! Casings derived from the words of the spec casing-collision rule (`IdentifierString` in the spec README).
//! The word split matches the `@codama/fragments` casing helpers, so both sides derive the same casings.

/// Joins the words, each word after the first capitalised (`MAX_SUPPLY` -> `maxSupply`, `getURL` -> `getUrl`).
/// Two identifiers of one sibling set collide when this form is the same.
pub fn camel_case(input: &str) -> String {
    split_words(input)
        .enumerate()
        .map(|(index, word)| match index {
            0 => word.to_ascii_lowercase(),
            _ => capitalize(word),
        })
        .collect()
}

/// Joins the lowercase words with `_` (`HTTPServer` -> `http_server`, `foo1Bar` -> `foo1_bar`).
pub fn snake_case(input: &str) -> String {
    split_words(input)
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join("_")
}

/// Splits a string into words with the spec word rule, keeping the input casing.
/// A word ends at any character outside `[A-Za-z0-9]` such as `_`, and empty words are discarded.
fn split_words(input: &str) -> impl Iterator<Item = &str> {
    input
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|chunk| !chunk.is_empty())
        .flat_map(split_at_case_boundaries)
}

/// Splits an ASCII alphanumeric chunk before each byte that starts a word.
fn split_at_case_boundaries(chunk: &str) -> Vec<&str> {
    let bytes = chunk.as_bytes();
    let (mut words, start) = (1..bytes.len()).fold((Vec::new(), 0), |(mut words, start), index| {
        if is_case_boundary(bytes[index - 1], bytes[index], bytes.get(index + 1)) {
            words.push(&chunk[start..index]);
            (words, index)
        } else {
            (words, start)
        }
    });
    words.push(&chunk[start..]);
    words
}

/// An uppercase letter starts a word after a lowercase letter or a digit (`foo1Bar` -> `foo1`, `Bar`).
/// It also starts a word after an uppercase letter when a lowercase letter follows (`HTTPServer` -> `HTTP`, `Server`).
/// A digit never starts a word (`foo1bar` is one word).
fn is_case_boundary(previous: u8, current: u8, next: Option<&u8>) -> bool {
    current.is_ascii_uppercase()
        && (previous.is_ascii_lowercase()
            || previous.is_ascii_digit()
            || (previous.is_ascii_uppercase() && next.is_some_and(u8::is_ascii_lowercase)))
}

fn capitalize(word: &str) -> String {
    let mut capitalized = word.to_ascii_lowercase();
    if let Some(first) = capitalized.get_mut(..1) {
        first.make_ascii_uppercase();
    }
    capitalized
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_words(input: &str, expected: &[&str]) {
        let words: Vec<String> = split_words(input).map(str::to_ascii_lowercase).collect();
        assert_eq!(words, expected, "words of {input:?}");
    }

    #[test]
    fn spec_examples() {
        assert_eq!(camel_case("MAX_SUPPLY"), "maxSupply");
        assert_eq!(camel_case("getURL"), "getUrl");
        assert_eq!(camel_case("fooBar"), camel_case("foo_bar"));
        assert_eq!(camel_case("foo1"), camel_case("foo_1"));
        assert_eq!(camel_case("getURL"), camel_case("get_url"));
        assert_eq!(camel_case("_foo"), camel_case("foo"));
        assert_ne!(camel_case("foo_dart"), camel_case("food_art"));
        assert_ne!(
            camel_case("group__sub_group__name"),
            camel_case("group_subgroup_name")
        );
    }

    #[test]
    fn pinned_words() {
        assert_words("HTTPServer", &["http", "server"]);
        assert_eq!(
            split_words("HTTPServer").collect::<Vec<_>>(),
            ["HTTP", "Server"]
        );
        assert_words("foo1bar", &["foo1bar"]);
        assert_words("foo1Bar", &["foo1", "bar"]);
        assert_words("MAX_SUPPLY", &["max", "supply"]);
        assert_words("getURL", &["get", "url"]);
        assert_words("_foo", &["foo"]);
        assert_words("foo__bar", &["foo", "bar"]);
        assert_words("group__sub_group__name", &["group", "sub", "group", "name"]);
        assert_words("", &[]);
        assert_words("___", &[]);
    }

    #[test]
    fn pinned_casings() {
        assert_eq!(snake_case("foo1Bar"), "foo1_bar");
        assert_eq!(snake_case("HTTPServer"), "http_server");
        assert_eq!(camel_case("HTTPServer"), "httpServer");
        assert_eq!(camel_case("foo1bar"), "foo1bar");
        assert_eq!(camel_case("_foo"), "foo");
        assert_eq!(snake_case("foo__bar"), "foo_bar");
        assert_eq!(camel_case("foo__bar"), "fooBar");
    }

    #[test]
    fn word_splitting_vectors() {
        let cases: [(&str, &[&str]); 8] = [
            ("MAX_SUPPLY", &["max", "supply"]),
            ("HTTPServer", &["http", "server"]),
            ("getURL", &["get", "url"]),
            ("_foo", &["foo"]),
            ("foo__bar", &["foo", "bar"]),
            ("foo1bar", &["foo1bar"]),
            ("foo1Bar", &["foo1", "bar"]),
            ("group__sub_group__name", &["group", "sub", "group", "name"]),
        ];
        for (input, expected) in cases {
            assert_words(input, expected);
            assert_eq!(snake_case(input), expected.join("_"));
            let camel: String = expected
                .iter()
                .enumerate()
                .map(|(index, word)| match index {
                    0 => word.to_string(),
                    _ => capitalize(word),
                })
                .collect();
            assert_eq!(camel_case(input), camel);
        }
    }

    #[test]
    fn same_camel_case_vectors() {
        let cases = [
            ("fooBar", "foo_bar"),
            ("foo1", "foo_1"),
            ("getURL", "get_url"),
            ("HTTPServer", "httpServer"),
            ("MAX_SUPPLY", "maxSupply"),
            ("_foo", "foo"),
            ("foo__bar", "foo_bar"),
            ("foo1Bar", "foo1_bar"),
        ];
        for (first, second) in cases {
            assert_eq!(
                camel_case(first),
                camel_case(second),
                "{first} and {second}"
            );
        }
    }

    #[test]
    fn distinct_camel_case_vectors() {
        let cases = [
            ("foo_dart", "food_art"),
            ("group__sub_group__name", "group_subgroup_name"),
            ("foo1bar", "foo1_bar"),
            ("HTTPServer", "hTTPServer"),
        ];
        for (first, second) in cases {
            assert_ne!(
                camel_case(first),
                camel_case(second),
                "{first} and {second}"
            );
        }
    }

    #[test]
    fn camel_case_casing() {
        assert_eq!(camel_case("lowercased"), "lowercased");
        assert_eq!(camel_case("UPPERCASED"), "uppercased");
        assert_eq!(camel_case("Capitalized"), "capitalized");
        assert_eq!(camel_case(""), "");
    }

    #[test]
    fn camel_case_numbers() {
        assert_eq!(
            camel_case("1before after2 bet3ween"),
            "1beforeAfter2Bet3ween"
        );
        assert_eq!(
            camel_case("50m3 1 2 3 numb3rs 8 3v3rywh3r3 9"),
            "50m3123Numb3rs83v3rywh3r39"
        );
    }

    #[test]
    fn camel_case_special_characters() {
        assert_eq!(
            camel_case("some::special\\\\chars+++in=between"),
            "someSpecialCharsInBetween"
        );
        assert_eq!(camel_case("$peçia!::ch*rs"), "peIaChRs");
        assert_eq!(camel_case("multiple.........dots"), "multipleDots");
        assert_eq!(camel_case("multiple---------dashes"), "multipleDashes");
        assert_eq!(
            camel_case("multiple_________underscores"),
            "multipleUnderscores"
        );
    }

    #[test]
    fn camel_case_from_snake_case() {
        assert_eq!(
            camel_case("from_lowercased_snake_case"),
            "fromLowercasedSnakeCase"
        );
        assert_eq!(
            camel_case("From_Capitalized_Snake_Case"),
            "fromCapitalizedSnakeCase"
        );
        assert_eq!(
            camel_case("FROM_UPPERCASED_SNAKE_CASE"),
            "fromUppercasedSnakeCase"
        );
        assert_eq!(
            camel_case("fr0m_5nak3_c4s3_w1th_42n_numb3r5"),
            "fr0m5nak3C4s3W1th42nNumb3r5"
        );
        assert_eq!(
            camel_case("frøm_snake_case_w:th_$peçia!_ch*rs"),
            "frMSnakeCaseWThPeIaChRs"
        );
        assert_eq!(
            camel_case(&camel_case("frøm_d0ubl3_Snake_c*se")),
            "frMd0ubl3SnakeCSe"
        );
    }

    #[test]
    fn camel_case_from_title_case() {
        assert_eq!(
            camel_case("from lowercased title case"),
            "fromLowercasedTitleCase"
        );
        assert_eq!(
            camel_case("From Capitalized Title Case"),
            "fromCapitalizedTitleCase"
        );
        assert_eq!(
            camel_case("FROM UPPERCASED TITLE CASE"),
            "fromUppercasedTitleCase"
        );
        assert_eq!(
            camel_case("Fr0m T1tl3 C4s3 W1th 42n Numb3r5"),
            "fr0mT1tl3C4s3W1th42nNumb3r5"
        );
        assert_eq!(
            camel_case("Frøm Title Case W:th $peçia! Ch*rs"),
            "frMTitleCaseWThPeIaChRs"
        );
        assert_eq!(
            camel_case(&camel_case("Frøm D0ubl3 Title C*se")),
            "frMd0ubl3TitleCSe"
        );
    }

    #[test]
    fn camel_case_from_pascal_case() {
        assert_eq!(camel_case("FromPascaleCase"), "fromPascaleCase");
        assert_eq!(
            camel_case("Fr0mP45c4l3C4s3W1th42nNumb3r5"),
            "fr0mP45c4l3C4s3W1th42nNumb3r5"
        );
        assert_eq!(
            camel_case("FrømPascaleCaseW:th$peçia!Ch*rs"),
            "frMPascaleCaseWThPeIaChRs"
        );
        assert_eq!(
            camel_case(&camel_case("FrømD0ubl3PascaleC*se")),
            "frMd0ubl3PascaleCSe"
        );
    }

    #[test]
    fn camel_case_from_camel_case() {
        assert_eq!(camel_case("FromCamelCase"), "fromCamelCase");
        assert_eq!(
            camel_case("Fr0mC4m3lC4s3W1th42nNumb3r5"),
            "fr0mC4m3lC4s3W1th42nNumb3r5"
        );
        assert_eq!(
            camel_case("FrømCamelCaseW:th$peçia!Ch*rs"),
            "frMCamelCaseWThPeIaChRs"
        );
        assert_eq!(
            camel_case(&camel_case("FrømD0ubl3CamelC*se")),
            "frMd0ubl3CamelCSe"
        );
    }

    #[test]
    fn camel_case_from_paths() {
        assert_eq!(
            camel_case("crate::my_module::my_type"),
            "crateMyModuleMyType"
        );
        assert_eq!(
            camel_case("/Users/username/My File.txt"),
            "usersUsernameMyFileTxt"
        );
        assert_eq!(
            camel_case("C:\\Users\\username\\My\\ File.txt"),
            "cUsersUsernameMyFileTxt"
        );
    }

    #[test]
    fn snake_case_casing() {
        assert_eq!(snake_case("lowercased"), "lowercased");
        assert_eq!(snake_case("UPPERCASED"), "uppercased");
        assert_eq!(snake_case("Capitalized"), "capitalized");
        assert_eq!(snake_case(""), "");
    }

    #[test]
    fn snake_case_numbers() {
        assert_eq!(
            snake_case("1before after2 bet3ween"),
            "1before_after2_bet3ween"
        );
        assert_eq!(
            snake_case("50m3 1 2 3 numb3rs 8 3v3rywh3r3 9"),
            "50m3_1_2_3_numb3rs_8_3v3rywh3r3_9"
        );
        assert_eq!(
            snake_case(&snake_case("50m3 1 2 3 numb3rs 8 3v3rywh3r3 9")),
            "50m3_1_2_3_numb3rs_8_3v3rywh3r3_9"
        );
    }

    #[test]
    fn snake_case_special_characters() {
        assert_eq!(
            snake_case("some::special\\\\chars+++in=between"),
            "some_special_chars_in_between"
        );
        assert_eq!(snake_case("$peçia!::ch*rs"), "pe_ia_ch_rs");
        assert_eq!(snake_case("multiple.........dots"), "multiple_dots");
        assert_eq!(snake_case("multiple---------dashes"), "multiple_dashes");
        assert_eq!(
            snake_case("multiple_________underscores"),
            "multiple_underscores"
        );
    }

    #[test]
    fn snake_case_from_snake_case() {
        assert_eq!(
            snake_case("from_lowercased_snake_case"),
            "from_lowercased_snake_case"
        );
        assert_eq!(
            snake_case("From_Capitalized_Snake_Case"),
            "from_capitalized_snake_case"
        );
        assert_eq!(
            snake_case("FROM_UPPERCASED_SNAKE_CASE"),
            "from_uppercased_snake_case"
        );
        assert_eq!(
            snake_case("fr0m_5nak3_c4s3_w1th_42n_numb3r5"),
            "fr0m_5nak3_c4s3_w1th_42n_numb3r5"
        );
        assert_eq!(
            snake_case("frøm_snake_case_w:th_$peçia!_ch*rs"),
            "fr_m_snake_case_w_th_pe_ia_ch_rs"
        );
        assert_eq!(
            snake_case(&snake_case("frøm_d0ubl3_Snake_c*se")),
            "fr_m_d0ubl3_snake_c_se"
        );
    }

    #[test]
    fn snake_case_from_title_case() {
        assert_eq!(
            snake_case("from lowercased title case"),
            "from_lowercased_title_case"
        );
        assert_eq!(
            snake_case("From Capitalized Title Case"),
            "from_capitalized_title_case"
        );
        assert_eq!(
            snake_case("FROM UPPERCASED TITLE CASE"),
            "from_uppercased_title_case"
        );
        assert_eq!(
            snake_case("Fr0m T1tl3 C4s3 W1th 42n Numb3r5"),
            "fr0m_t1tl3_c4s3_w1th_42n_numb3r5"
        );
        assert_eq!(
            snake_case("Frøm Title Case W:th $peçia! Ch*rs"),
            "fr_m_title_case_w_th_pe_ia_ch_rs"
        );
        assert_eq!(
            snake_case(&snake_case("Frøm D0ubl3 Title C*se")),
            "fr_m_d0ubl3_title_c_se"
        );
    }

    #[test]
    fn snake_case_from_pascal_case() {
        assert_eq!(snake_case("FromPascaleCase"), "from_pascale_case");
        assert_eq!(
            snake_case("Fr0mP45c4l3C4s3W1th42nNumb3r5"),
            "fr0m_p45c4l3_c4s3_w1th42n_numb3r5"
        );
        assert_eq!(
            snake_case("FrømPascaleCaseW:th$peçia!Ch*rs"),
            "fr_m_pascale_case_w_th_pe_ia_ch_rs"
        );
        assert_eq!(
            snake_case(&snake_case("FrømD0ubl3PascaleC*se")),
            "fr_m_d0ubl3_pascale_c_se"
        );
    }

    #[test]
    fn snake_case_from_camel_case() {
        assert_eq!(snake_case("FromCamelCase"), "from_camel_case");
        assert_eq!(
            snake_case("Fr0mC4m3lC4s3W1th42nNumb3r5"),
            "fr0m_c4m3l_c4s3_w1th42n_numb3r5"
        );
        assert_eq!(
            snake_case("FrømCamelCaseW:th$peçia!Ch*rs"),
            "fr_m_camel_case_w_th_pe_ia_ch_rs"
        );
        assert_eq!(
            snake_case(&snake_case("FrømD0ubl3CamelC*se")),
            "fr_m_d0ubl3_camel_c_se"
        );
    }

    #[test]
    fn snake_case_from_paths() {
        assert_eq!(
            snake_case("crate::my_module::my_type"),
            "crate_my_module_my_type"
        );
        assert_eq!(
            snake_case("/Users/username/My File.txt"),
            "users_username_my_file_txt"
        );
        assert_eq!(
            snake_case("C:\\Users\\username\\My\\ File.txt"),
            "c_users_username_my_file_txt"
        );
    }
}
