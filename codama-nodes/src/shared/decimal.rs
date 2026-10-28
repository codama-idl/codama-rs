use crate::Integer;
use serde::{de, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// A decimal string in the spec grammar: `-?(0|[1-9][0-9]*)("." [0-9]*[1-9])?`, `NaN`, `Infinity` or `-Infinity`.
/// The string keeps the value as written, also past the precision of an `f64`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Decimal(String);

impl Decimal {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The nearest `f64`. Distinct strings can round to the same `f64`.
    pub fn to_f64(&self) -> f64 {
        self.0.parse().expect("a decimal parses as f64")
    }

    /// Builds a decimal from number text in plain or exponent notation, such as `1.50`, `007.5` or `-2.5e-3`.
    /// Every digit stays: the zeros are trimmed and the exponent is expanded, with no `f64` rounding.
    /// The value must be within the `f64` range, which no float format exceeds.
    pub fn from_literal(literal: &str) -> Result<Self, ParseDecimalError> {
        let invalid = || ParseDecimalError(literal.to_string());
        let (sign, unsigned) = split_sign(literal);
        let (mantissa, exponent) = split_exponent(unsigned).ok_or_else(invalid)?;
        if !is_plain_number(mantissa) || !fits_f64(mantissa, exponent) {
            return Err(invalid());
        }
        Ok(Decimal(format!(
            "{sign}{}",
            plain_digits(mantissa, exponent)
        )))
    }
}

impl From<f64> for Decimal {
    fn from(value: f64) -> Self {
        if value.is_nan() {
            return Decimal("NaN".to_string());
        }

        let sign = if value.is_sign_negative() { "-" } else { "" };
        if value.is_infinite() {
            return Decimal(format!("{sign}Infinity"));
        }

        // ryu (through `serde_json`) writes the same shortest round-trip digits as JS `Number.toString`,
        // but may use an exponent (`1.5e-7`) or a trailing `.0`, which the spec grammar forbids.
        let shortest = serde_json::Number::from_f64(value.abs())
            .expect("the value is finite")
            .to_string();
        let (mantissa, exponent) = split_exponent(&shortest).expect("ryu writes an i32 exponent");
        Decimal(format!("{sign}{}", plain_digits(mantissa, exponent)))
    }
}

impl From<Integer> for Decimal {
    fn from(value: Integer) -> Self {
        Decimal(value.to_string())
    }
}

/// Splits a leading minus sign off number text, so `-1.5` gives `("-", "1.5")`.
fn split_sign(text: &str) -> (&str, &str) {
    match text.strip_prefix('-') {
        Some(unsigned) => ("-", unsigned),
        None => ("", text),
    }
}

/// Splits an exponent off number text, so `2.5e-3` gives `("2.5", -3)` and `2.5` gives `("2.5", 0)`.
/// Gives `None` when the exponent is not an `i32`, as in `1e` or `1e3.5`.
fn split_exponent(text: &str) -> Option<(&str, i32)> {
    match text.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => Some((mantissa, exponent.parse().ok()?)),
        None => Some((text, 0)),
    }
}

/// Digits with at most one `.` and a digit before it: `1.5`, `1.` and `007` pass, `.5` and `1.5.2` fail.
fn is_plain_number(text: &str) -> bool {
    let (integer, fraction) = text.split_once('.').unwrap_or((text, ""));
    !integer.is_empty() && is_digits(integer) && is_digits(fraction)
}

/// A finite `f64` that does not underflow to zero, so the exponent stays small enough to expand.
fn fits_f64(mantissa: &str, exponent: i32) -> bool {
    match format!("{mantissa}e{exponent}").parse::<f64>() {
        Ok(value) => value.is_finite() && (value != 0.0 || is_zero(mantissa)),
        Err(_) => false,
    }
}

/// Writes `mantissa` times ten to the power `exponent` as canonical digits, with no leading or trailing zeros.
/// A zero mantissa gives `0` without expanding the exponent.
fn plain_digits(mantissa: &str, exponent: i32) -> String {
    if is_zero(mantissa) {
        return "0".to_string();
    }
    let plain = expand_exponent(mantissa, exponent);
    let (integer, fraction) = plain.split_once('.').unwrap_or((&plain, ""));
    let integer = match integer.trim_start_matches('0') {
        "" => "0",
        integer => integer,
    };
    match fraction.trim_end_matches('0') {
        "" => integer.to_string(),
        fraction => format!("{integer}.{fraction}"),
    }
}

fn is_zero(mantissa: &str) -> bool {
    mantissa.bytes().all(|byte| matches!(byte, b'0' | b'.'))
}

fn is_digits(text: &str) -> bool {
    text.bytes().all(|byte| byte.is_ascii_digit())
}

/// Writes `mantissa` times ten to the power `exponent` in plain decimal notation.
/// Similar to JS implementation: `("1.5", -7)` -> `"0.00000015"`, `("1.5", 21)` -> `"1500000000000000000000"`.
fn expand_exponent(mantissa: &str, exponent: i32) -> String {
    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{integer}{fraction}");
    // The position of the decimal point in `digits`. Zero or less puts it before the first digit.
    let point = integer.len() as i32 + exponent;
    match usize::try_from(point) {
        Ok(point) if point >= digits.len() => {
            format!("{digits}{}", "0".repeat(point - digits.len()))
        }
        Ok(point) if point > 0 => {
            let (left, right) = digits.split_at(point);
            format!("{left}.{right}")
        }
        _ => format!("0.{}{digits}", "0".repeat(point.unsigned_abs() as usize)),
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDecimalError(String);

impl fmt::Display for ParseDecimalError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid decimal: {}", self.0)
    }
}

impl std::error::Error for ParseDecimalError {}

impl TryFrom<String> for Decimal {
    type Error = ParseDecimalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_canonical_decimal(&value) {
            Ok(Decimal(value))
        } else {
            Err(ParseDecimalError(value))
        }
    }
}

impl TryFrom<&str> for Decimal {
    type Error = ParseDecimalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Decimal::try_from(value.to_string())
    }
}

impl FromStr for Decimal {
    type Err = ParseDecimalError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Decimal::try_from(value)
    }
}

/// Checks `value` against the spec `decimal` grammar `-?(0|[1-9][0-9]*)("." [0-9]*[1-9])?` or a special.
/// Allows `0`, `-0`, `10`, `1.5`, `-0.051`, digits past `f64` precision, and `NaN`, `Infinity`, `-Infinity`.
/// Rejects leading zeros (`01`), trailing fraction zeros (`1.50`), `1e5`, `+1`, `.5`, `1.`, spaces, `inf`, `nan`.
fn is_canonical_decimal(value: &str) -> bool {
    if matches!(value, "NaN" | "Infinity" | "-Infinity") {
        return true;
    }
    let is_part = |part: &str| !part.is_empty() && is_digits(part);
    let (_, unsigned) = split_sign(value);
    let (integer, fraction) = match unsigned.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (unsigned, None),
    };
    let integer_ok = is_part(integer) && (integer == "0" || !integer.starts_with('0'));
    let fraction_ok = fraction.is_none_or(|fraction| is_part(fraction) && !fraction.ends_with('0'));
    integer_ok && fraction_ok
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DecimalVisitor;

        impl de::Visitor<'_> for DecimalVisitor {
            type Value = Decimal;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a canonical decimal string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Decimal::try_from(value)
                    .map_err(|_| E::invalid_value(de::Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_str(DecimalVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_f64() {
        for (value, expected) in [
            (0.0, "0"),
            (-0.0, "-0"),
            (1.0, "1"),
            (-0.051, "-0.051"),
            (0.1 + 0.2, "0.30000000000000004"),
            (602000000.0, "602000000"),
            (1e20, "100000000000000000000"),
            (1.5e21, "1500000000000000000000"),
            (1e-7, "0.0000001"),
            (-5e-7, "-0.0000005"),
            (f64::NAN, "NaN"),
            (-f64::NAN, "NaN"),
            (f64::INFINITY, "Infinity"),
            (f64::NEG_INFINITY, "-Infinity"),
        ] {
            assert_eq!(Decimal::from(value).as_str(), expected);
        }
    }

    #[test]
    fn from_f64_breaks_ties_to_even() {
        for (bits, expected) in [
            (0x42ebdac025474844, "245010723912258.12"),
            (0x430b7dd7d3580522, "967273786048676.2"),
            (0xc2b83a6261284d90, "-26639037704269.562"),
        ] {
            assert_eq!(Decimal::from(f64::from_bits(bits)).as_str(), expected);
        }
    }

    #[test]
    fn from_f64_max() {
        let max = Decimal::from(f64::MAX);
        assert_eq!(
            max.as_str(),
            format!("17976931348623157{}", "0".repeat(292))
        );
        assert_eq!(max.to_f64(), f64::MAX);
        let min = Decimal::from(5e-324);
        assert_eq!(min.as_str(), format!("0.{}5", "0".repeat(323)));
        assert_eq!(min.to_f64(), 5e-324);
    }

    #[test]
    fn from_integer() {
        for (value, expected) in [
            (Integer::from(0u8), "0"),
            (Integer::from(42u64), "42"),
            (Integer::from(-5i64), "-5"),
            (
                Integer::from(u128::MAX),
                "340282366920938463463374607431768211455",
            ),
            (
                Integer::from(i128::MIN),
                "-170141183460469231731687303715884105728",
            ),
        ] {
            let decimal = Decimal::from(value);
            assert_eq!(decimal.as_str(), expected);
            assert_eq!(Decimal::try_from(expected), Ok(decimal));
        }
    }

    #[test]
    fn from_literal() {
        for (literal, expected) in [
            ("1.5", "1.5"),
            ("-2.50", "-2.5"),
            ("007.50", "7.5"),
            ("1.", "1"),
            ("42", "42"),
            ("0.0", "0"),
            ("-0.0", "-0"),
            ("1e3", "1000"),
            ("1E3", "1000"),
            ("2.5e+2", "250"),
            ("1.5e-3", "0.0015"),
            ("0.5e3", "500"),
            ("123e-5", "0.00123"),
            ("0.12345678901234567890", "0.1234567890123456789"),
            ("0e999999999", "0"),
            ("-0.0e-999999999", "-0"),
        ] {
            assert_eq!(Decimal::from_literal(literal).unwrap().as_str(), expected);
        }
    }

    #[test]
    fn from_literal_rejects_values_outside_the_f64_range() {
        for literal in ["1e400", "-1e400", "1e-400", "1e2147483647", "1e-2147483648"] {
            let error = Decimal::from_literal(literal).unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid decimal: {literal}"));
        }
    }

    #[test]
    fn from_literal_rejects_non_numbers() {
        for literal in [
            "", "-", "+1", ".5", "1e", "1e+", "1.5.2", "1e3.5", "1_000", "0x10", "NaN", " 1",
        ] {
            let error = Decimal::from_literal(literal).unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid decimal: {literal}"));
        }
    }

    #[test]
    fn to_f64() {
        let to_f64 = |value: &str| value.parse::<Decimal>().unwrap().to_f64();
        assert_eq!(to_f64("-0.051"), -0.051);
        assert!(to_f64("-0").is_sign_negative());
        assert!(to_f64("NaN").is_nan());
        assert_eq!(to_f64("Infinity"), f64::INFINITY);
        assert_eq!(to_f64("-Infinity"), f64::NEG_INFINITY);
    }

    #[test]
    fn from_str_keeps_digits_past_f64_precision() {
        let decimal = "0.1000000000000000000000000001".parse::<Decimal>().unwrap();
        assert_eq!(decimal.as_str(), "0.1000000000000000000000000001");
        assert_eq!(decimal.to_f64(), 0.1);
        assert_ne!(decimal, Decimal::from(0.1));
    }

    #[test]
    fn from_str_rejects_non_canonical_strings() {
        for value in [
            "1e5", "+1", "01", "-01", "1.50", "1.", ".5", "-", "inf", "nan", "-NaN", " 1", "",
        ] {
            let error = value.parse::<Decimal>().unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid decimal: {value}"));
        }
    }

    #[test]
    fn json_keeps_the_string() {
        let large = format!("1{}", "0".repeat(400));
        for value in ["1.5", "-0", "NaN", "Infinity", "-Infinity", large.as_str()] {
            let json = format!(r#""{value}""#);
            let decimal = serde_json::from_str::<Decimal>(&json).unwrap();
            assert_eq!(decimal.as_str(), value);
            assert_eq!(serde_json::to_string(&decimal).unwrap(), json);
        }
    }

    #[test]
    fn from_json_rejects_non_canonical_strings() {
        let error = serde_json::from_str::<Decimal>(r#""1e5""#).unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid value: string \"1e5\", expected a canonical decimal string at line 1 column 5"
        );
        assert!(serde_json::from_str::<Decimal>("1.5").is_err());
    }
}
