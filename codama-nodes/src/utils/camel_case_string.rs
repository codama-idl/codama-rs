use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CamelCaseString(String);

impl CamelCaseString {
    pub fn new<T>(string: T) -> Self
    where
        T: Into<String>,
    {
        let string: String = string.into();
        Self(to_camel_case(&string))
    }
}

impl<T> From<T> for CamelCaseString
where
    T: Into<String>,
{
    fn from(string: T) -> Self {
        Self::new(string)
    }
}

impl Deref for CamelCaseString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
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
    fn new_from_character() {
        let value = CamelCaseString::new('a');
        assert_eq!(value.0, "a");
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
    fn from_character() {
        let value: CamelCaseString = 'a'.into();
        assert_eq!(value.0, "a");
    }

    #[test]
    fn deref() {
        let value = CamelCaseString::new("Hello World!");
        assert_eq!(*value, "helloWorld");
    }
}
