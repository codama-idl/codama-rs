/// Implements `as_str`, `Display`, `TryFrom`, `FromStr` and `Deserialize` for a constrained string type of the spec.
/// The type must be a tuple struct over a `String`, which holds the value as given.
/// `$is_valid` checks the grammar and `$error` wraps a rejected string.
/// `Deserialize` goes through `TryFrom`, so a JSON string must match the same grammar.
/// `$expecting` completes the serde message "expected ..." for a rejected JSON string.
macro_rules! impl_constrained_string {
    ($type:ident, $is_valid:path, $error:path, $expecting:literal) => {
        impl $type {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl ::std::fmt::Display for $type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl TryFrom<String> for $type {
            type Error = ::codama_errors::CodamaError;

            fn try_from(value: String) -> ::codama_errors::CodamaResult<Self> {
                if $is_valid(&value) {
                    Ok(Self(value))
                } else {
                    Err($error(value))
                }
            }
        }

        impl TryFrom<&str> for $type {
            type Error = ::codama_errors::CodamaError;

            fn try_from(value: &str) -> ::codama_errors::CodamaResult<Self> {
                Self::try_from(value.to_string())
            }
        }

        impl ::std::str::FromStr for $type {
            type Err = ::codama_errors::CodamaError;

            fn from_str(value: &str) -> ::codama_errors::CodamaResult<Self> {
                Self::try_from(value)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct StringVisitor;

                impl ::serde::de::Visitor<'_> for StringVisitor {
                    type Value = $type;

                    fn expecting(
                        &self,
                        formatter: &mut ::std::fmt::Formatter,
                    ) -> ::std::fmt::Result {
                        formatter.write_str($expecting)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        $type::try_from(value).map_err(|_| {
                            E::invalid_value(::serde::de::Unexpected::Str(value), &self)
                        })
                    }
                }

                deserializer.deserialize_str(StringVisitor)
            }
        }
    };
}

pub(super) use impl_constrained_string;

/// Strips a leading identifier `[A-Za-z_][A-Za-z0-9_]*` and returns the rest.
pub(super) fn strip_identifier(value: &str) -> Option<&str> {
    let first_ok =
        value.starts_with(|character: char| character.is_ascii_alphabetic() || character == '_');
    let end = value
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(value.len());
    first_ok.then(|| &value[end..])
}

/// Strips a leading index `"[" (0|[1-9][0-9]*) "]"` and returns the rest.
pub(super) fn strip_index(value: &str) -> Option<&str> {
    let (integer, rest) = value.strip_prefix('[')?.split_once(']')?;
    let is_digits = !integer.is_empty() && integer.bytes().all(|byte| byte.is_ascii_digit());
    let is_integer = is_digits && (integer == "0" || !integer.starts_with('0'));
    is_integer.then_some(rest)
}
