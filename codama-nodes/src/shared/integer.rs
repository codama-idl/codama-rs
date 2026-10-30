use serde::{de, Deserialize, Serialize};
use std::fmt;

// `From` and `Deserialize` map every value of zero and above to `Unsigned`, so the integers they build
// have one representation each and the derived `PartialEq` compares their values.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Integer {
    Unsigned(u128),
    Signed(i128),
}

macro_rules! impl_from_unsigned {
    ($($type:ty),*) => {$(
        impl From<$type> for Integer {
            fn from(value: $type) -> Self {
                Integer::Unsigned(u128::from(value))
            }
        }
    )*};
}

macro_rules! impl_from_signed {
    ($($type:ty),*) => {$(
        impl From<$type> for Integer {
            fn from(value: $type) -> Self {
                Integer::from(i128::from(value))
            }
        }
    )*};
}

impl_from_unsigned!(u8, u16, u32, u64);
impl_from_signed!(i8, i16, i32, i64);

impl From<u128> for Integer {
    fn from(value: u128) -> Self {
        Integer::Unsigned(value)
    }
}

impl From<i128> for Integer {
    fn from(value: i128) -> Self {
        match u128::try_from(value) {
            Ok(unsigned) => Integer::Unsigned(unsigned),
            Err(_) => Integer::Signed(value),
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Integer::Unsigned(value) => value.fmt(formatter),
            Integer::Signed(value) => value.fmt(formatter),
        }
    }
}

// The spec writes integers as base-10 strings, so values up to 128 bits survive JSON transport.
impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IntegerVisitor;

        impl de::Visitor<'_> for IntegerVisitor {
            type Value = Integer;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a canonical base-10 integer string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let integer = match value.strip_prefix('-') {
                    Some(_) => value.parse::<i128>().ok().map(Integer::from),
                    None => value.parse::<u128>().ok().map(Integer::from),
                };
                // The spec grammar gives every integer one spelling, so `+1`, `007` and `-0` are invalid.
                match integer {
                    Some(integer) if integer.to_string() == value => Ok(integer),
                    _ => Err(E::invalid_value(de::Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(IntegerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_unsigned() {
        assert_eq!(Integer::from(42u8), Integer::Unsigned(42));
        assert_eq!(Integer::from(42u16), Integer::Unsigned(42));
        assert_eq!(Integer::from(42u32), Integer::Unsigned(42));
        assert_eq!(Integer::from(42u64), Integer::Unsigned(42));
        assert_eq!(Integer::from(u128::MAX), Integer::Unsigned(u128::MAX));
    }

    #[test]
    fn from_signed() {
        assert_eq!(Integer::from(-42i8), Integer::Signed(-42));
        assert_eq!(Integer::from(-42i16), Integer::Signed(-42));
        assert_eq!(Integer::from(-42i32), Integer::Signed(-42));
        assert_eq!(Integer::from(-42i64), Integer::Signed(-42));
        assert_eq!(Integer::from(i128::MIN), Integer::Signed(i128::MIN));
    }

    #[test]
    fn from_non_negative_signed() {
        assert_eq!(Integer::from(42), Integer::Unsigned(42));
        assert_eq!(Integer::from(0i64), Integer::Unsigned(0));
    }

    #[test]
    fn to_json() {
        assert_eq!(
            serde_json::to_string(&Integer::from(42)).unwrap(),
            r#""42""#
        );
        assert_eq!(
            serde_json::to_string(&Integer::from(-42)).unwrap(),
            r#""-42""#
        );
        assert_eq!(
            serde_json::to_string(&Integer::from(u128::MAX)).unwrap(),
            r#""340282366920938463463374607431768211455""#
        );
    }

    #[test]
    fn from_json() {
        let from_json = |json: &str| serde_json::from_str::<Integer>(json).unwrap();
        assert_eq!(from_json(r#""0""#), Integer::Unsigned(0));
        assert_eq!(from_json(r#""42""#), Integer::Unsigned(42));
        assert_eq!(from_json(r#""-42""#), Integer::Signed(-42));
        assert_eq!(
            from_json(r#""-170141183460469231731687303715884105728""#),
            Integer::Signed(i128::MIN)
        );
    }

    #[test]
    fn from_json_rejects_non_canonical_strings() {
        for json in [
            r#""+1""#, r#""007""#, r#""-0""#, r#"" 1""#, r#""1.0""#, r#""""#, "42",
        ] {
            assert!(serde_json::from_str::<Integer>(json).is_err(), "{json}");
        }
    }

    #[test]
    fn from_json_rejects_values_beyond_128_bits() {
        let json = r#""340282366920938463463374607431768211456""#;
        let error = serde_json::from_str::<Integer>(json).unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid value: string \"340282366920938463463374607431768211456\", expected a canonical base-10 integer string at line 1 column 41"
        );
    }
}
