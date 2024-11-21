use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Docs(Vec<String>);

impl Docs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push<T: Into<String>>(&mut self, value: T) {
        self.0.push(value.into());
    }
}

impl From<Vec<String>> for Docs {
    fn from(vec: Vec<String>) -> Self {
        Self(vec)
    }
}

impl Deref for Docs {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Docs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[String]> for Docs {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}

impl AsMut<[String]> for Docs {
    fn as_mut(&mut self) -> &mut [String] {
        &mut self.0
    }
}

impl Index<usize> for Docs {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Docs {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let docs = Docs::new();
        assert_eq!(docs.0, Vec::<String>::new());
    }

    #[test]
    fn new_and_push() {
        let mut docs = Docs::new();
        docs.push("Hello");
        docs.push(String::from("World"));
        docs.push('!');
        assert_eq!(
            docs.0,
            vec!["Hello".to_string(), "World".to_string(), "!".to_string()]
        );
    }

    #[test]
    fn from_vec() {
        let docs: Docs = vec!["Hello".to_string(), "World".to_string()].into();
        assert_eq!(docs.0, vec!["Hello".to_string(), "World".to_string()]);
    }
}
