use super::string::strip_identifier;
use serde::{de, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// A plugin namespace in the spec `namespace` grammar, such as `anchor`, `i18n.es` or `codama.resolver`.
/// The string keeps the value as written, it is never re-cased.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Namespace(String);

impl Namespace {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNamespaceError(String);

impl fmt::Display for ParseNamespaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid namespace: {}", self.0)
    }
}

impl std::error::Error for ParseNamespaceError {}

impl TryFrom<String> for Namespace {
    type Error = ParseNamespaceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_namespace(&value) {
            Ok(Namespace(value))
        } else {
            Err(ParseNamespaceError(value))
        }
    }
}

impl TryFrom<&str> for Namespace {
    type Error = ParseNamespaceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Namespace::try_from(value.to_string())
    }
}

impl FromStr for Namespace {
    type Err = ParseNamespaceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Namespace::try_from(value)
    }
}

/// Checks `value` against the spec `namespace` grammar.
/// For example `anchor`, `i18n.es` or `codama.resolver`.
/// See: @codama/spec/api/types.ts.
fn is_namespace(value: &str) -> bool {
    value
        .split('.')
        .all(|segment| strip_identifier(segment) == Some(""))
}

impl<'de> Deserialize<'de> for Namespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NamespaceVisitor;

        impl de::Visitor<'_> for NamespaceVisitor {
            type Value = Namespace;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a namespace string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Namespace::try_from(value)
                    .map_err(|_| E::invalid_value(de::Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_str(NamespaceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: [&str; 9] = [
        "anchor",
        "i18n",
        "i18n.es",
        "codama.resolver",
        "codama.extraArgument",
        "my_crate.my_plugin",
        "_private",
        "_a.B1",
        "a.b.c",
    ];

    const INVALID: [&str; 15] = [
        "",
        ".",
        ".a",
        "a.",
        "a..b",
        "1a",
        "a.1b",
        "a-b",
        "codama-rs",
        "a b",
        " a",
        "a[0]",
        "a\n",
        "\u{e9}t\u{e9}",
        "a.\u{e9}",
    ];

    #[test]
    fn from_str() {
        for value in VALID {
            let namespace = value.parse::<Namespace>().unwrap();
            assert_eq!(namespace.as_str(), value);
            assert_eq!(namespace.to_string(), value);
        }
    }

    #[test]
    fn from_str_rejects_invalid_namespaces() {
        for value in INVALID {
            let error = value.parse::<Namespace>().unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid namespace: {value}"));
        }
    }

    #[test]
    fn try_from() {
        assert_eq!(Namespace::try_from("i18n.es").unwrap().as_str(), "i18n.es");
        assert_eq!(
            Namespace::try_from("i18n.es".to_string()).unwrap().as_str(),
            "i18n.es"
        );
        assert_eq!(
            Namespace::try_from("i18n..es".to_string()),
            Err(ParseNamespaceError("i18n..es".to_string()))
        );
    }

    #[test]
    fn json_keeps_the_string() {
        for value in VALID {
            let json = serde_json::to_string(value).unwrap();
            let namespace = serde_json::from_str::<Namespace>(&json).unwrap();
            assert_eq!(namespace.as_str(), value);
            assert_eq!(serde_json::to_string(&namespace).unwrap(), json);
        }
    }

    #[test]
    fn from_json_rejects_invalid_namespaces() {
        for value in INVALID {
            let json = serde_json::to_string(value).unwrap();
            assert!(serde_json::from_str::<Namespace>(&json).is_err());
        }
        assert!(serde_json::from_str::<Namespace>("0").is_err());
    }

    #[test]
    fn from_json_error_keeps_the_position() {
        let error = serde_json::from_str::<Namespace>(r#""a..b""#).unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid value: string \"a..b\", expected a namespace string at line 1 column 6"
        );
    }
}
