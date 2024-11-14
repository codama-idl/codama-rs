use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    for word in input.split(|c: char| !c.is_alphanumeric()) {
        if word.is_empty() {
            continue;
        }

        if result.is_empty() {
            // Keep the first word in lowercase
            result.push_str(&word.to_lowercase());
        } else {
            // Capitalize the first letter and lowercase the rest
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_ascii_uppercase());
                result.extend(chars.flat_map(|c| c.to_lowercase()));
            }
        }
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
        assert_eq!(value.0, "this123Str1ngHas456nNumbers");
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
}
