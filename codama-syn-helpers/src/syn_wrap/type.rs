use super::Path;
use codama_errors::CodamaResult;
use std::ops::Deref;

pub struct Type<'a>(pub &'a syn::Type);

impl Type<'_> {
    pub fn try_as_path(&self) -> CodamaResult<Path> {
        match self.0 {
            syn::Type::Path(path) => Ok(Path(&path.path)),
            _ => Err(syn::Error::new_spanned(self.0, "expected a path").into()),
        }
    }

    pub fn as_path(&self) -> Path {
        self.try_as_path().unwrap()
    }

    pub fn is_path(&self, path: &str) -> bool {
        self.as_path().is(path)
    }

    pub fn is_strict_path(&self, path: &str) -> bool {
        self.as_path().is_strict(path)
    }
}

impl Deref for Type<'_> {
    type Target = syn::Type;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn as_path_ok() {
        let r#type = syn_build::parse(quote! { std::option::Option<String> });
        let r#type = Type(&r#type);
        let result = r#type.try_as_path();
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn as_path_err() {
        let r#type = syn_build::parse(quote! { [u8; 32] });
        let r#type = Type(&r#type);
        let result = r#type.try_as_path();
        assert!(matches!(result, Err(_)));
    }
}
