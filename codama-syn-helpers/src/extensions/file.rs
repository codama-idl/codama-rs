use syn::File;

pub trait FileExtension {
    fn get_self(&self) -> &File;

    /// Returns the string value of the Fileession if it is a literal string.
    fn empty() -> File {
        syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl FileExtension for File {
    fn get_self(&self) -> &File {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let file = File::empty();
        assert_eq!(file.shebang, None);
        assert!(file.attrs.is_empty());
        assert!(file.items.is_empty());
    }
}
