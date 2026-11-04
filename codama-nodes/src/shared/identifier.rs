use super::casing::snake_case;
use super::string::{impl_constrained_string, strip_identifier};
use codama_errors::{CodamaError, CodamaResult};
use serde::Serialize;

/// An identifier in the spec `identifier` grammar `[A-Za-z_][A-Za-z0-9_]*`, such as `amount` or `token_program`.
/// The string keeps the value as written, it is never re-cased.
/// The default value is empty and marks a name that is not set yet. It is not a valid identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize)]
#[serde(transparent)]
pub struct Identifier(String);

impl_constrained_string!(
    Identifier,
    is_identifier,
    CodamaError::InvalidIdentifier,
    "an identifier string"
);

impl Identifier {
    /// Identifier for a name from Rust code, such as a type, field, variant or crate name.
    /// The name is snake_cased with the spec word rule (`MyAccount` -> `my_account`, `my-program` -> `my_program`).
    /// A raw identifier loses its prefix (`r#type` -> `type`).
    pub fn from_rust_name(name: &str) -> CodamaResult<Self> {
        let unraw = name.strip_prefix("r#").unwrap_or(name);
        // The word split drops non-ASCII letters, so `été` would give `t` with no error.
        if !unraw.is_ascii() {
            return Err(CodamaError::InvalidIdentifier(name.to_string()));
        }
        Self::try_from(snake_case(unraw))
            .map_err(|_| CodamaError::InvalidIdentifier(name.to_string()))
    }

    /// Only the default value is empty, since `TryFrom` rejects an empty string.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Checks `value` against the spec `identifier` grammar.
/// For example `amount`, `tokenProgram` or `_reserved`.
/// See: @codama/spec/api/types.ts.
fn is_identifier(value: &str) -> bool {
    strip_identifier(value) == Some("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: [&str; 10] = [
        "amount",
        "tokenProgram",
        "token_program",
        "TokenProgram",
        "HTTPServer",
        "_reserved",
        "_",
        "a",
        "field_1",
        "u8",
    ];

    const INVALID: [&str; 12] = [
        "",
        "1a",
        "0",
        "a b",
        " a",
        "a ",
        "a-b",
        "a.b",
        "a[0]",
        "r#type",
        "a\n",
        "\u{e9}t\u{e9}",
    ];

    #[test]
    fn from_str() {
        for value in VALID {
            let identifier = value.parse::<Identifier>().unwrap();
            assert_eq!(identifier.as_str(), value);
            assert_eq!(identifier.to_string(), value);
        }
    }

    #[test]
    fn from_str_rejects_invalid_identifiers() {
        for value in INVALID {
            let error = value.parse::<Identifier>().unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid identifier: {value}"));
        }
    }

    #[test]
    fn try_from() {
        assert_eq!(
            Identifier::try_from("my_value").unwrap().as_str(),
            "my_value"
        );
        assert_eq!(
            Identifier::try_from("myValue".to_string())
                .unwrap()
                .as_str(),
            "myValue"
        );
        assert!(matches!(
            Identifier::try_from("my value".to_string()),
            Err(CodamaError::InvalidIdentifier(value)) if value == "my value"
        ));
    }

    #[test]
    fn from_rust_name_snake_cases_the_name() {
        for (name, expected) in [
            ("amount", "amount"),
            ("my_account", "my_account"),
            ("MyAccount", "my_account"),
            ("myAccount", "my_account"),
            ("HTTPServer", "http_server"),
            ("getURL", "get_url"),
            ("MAX_SUPPLY", "max_supply"),
            ("foo1Bar", "foo1_bar"),
            ("foo1bar", "foo1bar"),
            ("foo__bar", "foo_bar"),
            ("_reserved", "reserved"),
            ("my-program", "my_program"),
            ("arg0", "arg0"),
        ] {
            assert_eq!(Identifier::from_rust_name(name).unwrap().as_str(), expected);
        }
    }

    #[test]
    fn from_rust_name_strips_the_raw_prefix() {
        assert_eq!(
            Identifier::from_rust_name("r#type").unwrap().as_str(),
            "type"
        );
        assert_eq!(
            Identifier::from_rust_name("r#MyType").unwrap().as_str(),
            "my_type"
        );
    }

    #[test]
    fn from_rust_name_rejects_names_without_a_valid_identifier() {
        for name in [
            "",
            "__",
            "1st",
            "_1st",
            "\u{e9}t\u{e9}",
            "r#\u{e9}t\u{e9}",
            "caf\u{e9}",
        ] {
            let error = Identifier::from_rust_name(name).unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid identifier: {name}"));
        }
    }

    #[test]
    fn default_is_empty() {
        let identifier = Identifier::default();
        assert!(identifier.is_empty());
        assert_eq!(identifier.as_str(), "");
        assert!(!Identifier::try_from("a").unwrap().is_empty());
    }

    #[test]
    fn as_ref() {
        let identifier = Identifier::try_from("helloWorld").unwrap();
        assert_eq!(identifier.as_ref(), "helloWorld");
    }

    #[test]
    fn json_keeps_the_string() {
        for value in VALID {
            let json = serde_json::to_string(value).unwrap();
            let identifier = serde_json::from_str::<Identifier>(&json).unwrap();
            assert_eq!(identifier.as_str(), value);
            assert_eq!(serde_json::to_string(&identifier).unwrap(), json);
        }
    }

    #[test]
    fn from_json_rejects_invalid_identifiers() {
        for value in INVALID {
            let json = serde_json::to_string(value).unwrap();
            assert!(serde_json::from_str::<Identifier>(&json).is_err());
        }
        assert!(serde_json::from_str::<Identifier>("0").is_err());
    }

    #[test]
    fn from_json_error_keeps_the_position() {
        let error = serde_json::from_str::<Identifier>(r#""a-b""#).unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid value: string \"a-b\", expected an identifier string at line 1 column 5"
        );
    }

    #[test]
    fn default_to_json_is_empty() {
        let json = serde_json::to_string(&Identifier::default()).unwrap();
        assert_eq!(json, r#""""#);
    }
}
