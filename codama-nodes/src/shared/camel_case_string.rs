use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct CamelCaseString(String);

impl CamelCaseString {
    pub fn new<T>(string: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(to_camel_case(string.as_ref()))
    }
}

impl Into<String> for CamelCaseString {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for CamelCaseString {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl From<&str> for CamelCaseString {
    fn from(string: &str) -> Self {
        Self::new(string)
    }
}

impl Deref for CamelCaseString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for CamelCaseString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut new_word = true;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        if c.is_alphanumeric() {
            if new_word && !result.is_empty() {
                // Capitalize the first letter of each new word (except the first word)
                result.extend(c.to_uppercase());
            } else {
                // Lowercase the first letter of the first word and other letters
                result.extend(c.to_lowercase());
            }
            new_word = false;
        } else {
            new_word = true;
        }

        // Treat numbers as their own "words" to start a new word afterward
        if c.is_numeric() {
            new_word = true;
        }

        // Handle transitions from lowercase to uppercase (e.g., PascalCase)
        if i + 1 < chars.len() && c.is_lowercase() && chars[i + 1].is_uppercase() {
            new_word = true;
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
        let value = CamelCaseString::new(String::from("Hello This is a Long Title!"));
        assert_eq!(value.0, "helloThisIsALongTitle");
    }

    #[test]
    fn parse_from_numbers() {
        let value = CamelCaseString::new(String::from("This123 str1ng has 456n numbers"));
        assert_eq!(value.0, "this123Str1NgHas456NNumbers");
    }

    #[test]
    fn parse_from_snake_case() {
        let value = CamelCaseString::new(String::from("hello_this_is__a_snake_case"));
        assert_eq!(value.0, "helloThisIsASnakeCase");
    }

    #[test]
    fn parse_from_pascal_case() {
        let value = CamelCaseString::new(String::from("HelloThisIs7PascalCaseWords"));
        assert_eq!(value.0, "helloThisIs7PascalCaseWords");
    }

    #[test]
    fn double_parse() {
        let value = to_camel_case("my_value");
        let value = to_camel_case(&value);
        assert_eq!(value, "myValue");
    }

    #[test]
    fn new_from_string() {
        let value = CamelCaseString::new(String::from("my_value"));
        assert_eq!(value.0, "myValue");
    }

    #[test]
    fn new_from_str() {
        let value = CamelCaseString::new("my_value");
        assert_eq!(value.0, "myValue");
    }

    #[test]
    fn new_from_self() {
        let value = CamelCaseString::new(CamelCaseString::new("my_value"));
        assert_eq!(value.0, "myValue");
    }

    #[test]
    fn from_string() {
        let value: CamelCaseString = String::from("my_value").into();
        assert_eq!(value.0, "myValue");
    }

    #[test]
    fn from_str() {
        let value: CamelCaseString = "my_value".into();
        assert_eq!(value.0, "myValue");
    }

    #[test]
    fn into_string() {
        let value: String = CamelCaseString::new("my_value").into();
        assert_eq!(value, "myValue");
    }

    #[test]
    fn deref() {
        let value = CamelCaseString::new("Hello World!");
        assert_eq!(*value, "helloWorld");
    }

    #[test]
    fn as_ref() {
        let value = CamelCaseString::new("Hello World!");
        assert_eq!(value.as_ref(), "helloWorld");
    }

    #[test]
    fn to_json() {
        let value = CamelCaseString::new("helloWorld");
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"helloWorld\"");
    }

    #[test]
    fn from_json() {
        let json = "\"helloWorld\"";
        let value: CamelCaseString = serde_json::from_str(json).unwrap();
        assert_eq!(value, CamelCaseString::new("helloWorld"));
    }
}
