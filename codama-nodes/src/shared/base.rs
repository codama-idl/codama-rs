use crate::Base;
use codama_errors::{CodamaError, CodamaResult};
use serde::{de, Deserialize, Serialize};

impl TryFrom<u64> for Base {
    type Error = CodamaError;

    fn try_from(value: u64) -> CodamaResult<Self> {
        match value {
            2 => Ok(Base::Base2),
            10 => Ok(Base::Base10),
            _ => Err(CodamaError::InvalidBase(value)),
        }
    }
}

// Spec writes `base` as JSON number `2` or `10`, which serde enum derives cannot express.
impl Serialize for Base {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Base::Base2 => serializer.serialize_u8(2),
            Base::Base10 => serializer.serialize_u8(10),
        }
    }
}

impl<'de> Deserialize<'de> for Base {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BaseVisitor;

        impl de::Visitor<'_> for BaseVisitor {
            type Value = Base;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("the number 2 or 10")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Base::try_from(value)
                    .map_err(|_| E::invalid_value(de::Unexpected::Unsigned(value), &self))
            }
        }

        deserializer.deserialize_u64(BaseVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        for (value, expected) in [(2, Base::Base2), (10, Base::Base10)] {
            assert_eq!(Base::try_from(value).unwrap(), expected);
        }
        for value in [0, 3, 16] {
            assert_eq!(
                Base::try_from(value).unwrap_err().to_string(),
                format!("Invalid base: {value}")
            );
        }
    }

    #[test]
    fn to_json() {
        assert_eq!(serde_json::to_string(&Base::Base2).unwrap(), "2");
        assert_eq!(serde_json::to_string(&Base::Base10).unwrap(), "10");
    }

    #[test]
    fn from_json() {
        assert_eq!(serde_json::from_str::<Base>("2").unwrap(), Base::Base2);
        assert_eq!(serde_json::from_str::<Base>("10").unwrap(), Base::Base10);
    }

    #[test]
    fn from_json_rejects_other_values() {
        for json in ["3", "-2", "2.0", r#""2""#] {
            assert!(serde_json::from_str::<Base>(json).is_err(), "{json}");
        }
    }
}
