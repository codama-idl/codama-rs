use super::Path;
use codama_errors::CodamaResult;
use std::ops::Deref;

pub struct Type<'a>(pub &'a syn::Type);

impl<'a> Type<'a> {
    pub fn as_path(&self) -> CodamaResult<Path<'a>> {
        match self.0 {
            syn::Type::Path(path) => Ok(Path(&path.path)),
            _ => Err(syn::Error::new_spanned(self.0, "expected a path").into()),
        }
    }

    pub fn is_path(&self, path: &str) -> bool {
        match self.as_path() {
            Ok(p) => p.is(path),
            _ => false,
        }
    }

    pub fn is_strict_path(&self, path: &str) -> bool {
        match self.as_path() {
            Ok(p) => p.is_strict(path),
            _ => false,
        }
    }

    /// Shortcut for checking if the type is an expected path and returning the first generic type.
    /// This also ensures there is only one generic type to unwrap.
    pub fn unwrap_inner_type(&self, path: &str) -> CodamaResult<&'a syn::Type> {
        self.as_path()?.unwrap_inner_type(path)
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
        assert!(matches!(Type(&r#type).as_path(), Ok(_)));
    }

    #[test]
    fn as_path_err() {
        let r#type = syn_build::parse(quote! { [u8; 32] });
        assert!(matches!(Type(&r#type).as_path(), Err(_)));
    }
}
