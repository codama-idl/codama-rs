use super::Path;
use codama_errors::CodamaResult;

pub trait Type {
    fn get_self(&self) -> &syn::Type;

    fn as_path(&self) -> CodamaResult<&syn::Path> {
        let this = self.get_self();
        match this {
            syn::Type::Path(path) => Ok(&path.path),
            _ => Err(syn::Error::new_spanned(this, "expected a path").into()),
        }
    }

    fn is_path(&self, path: &str) -> bool {
        match self.as_path() {
            Ok(p) => p.is(path),
            _ => false,
        }
    }

    fn is_strict_path(&self, path: &str) -> bool {
        match self.as_path() {
            Ok(p) => p.is_strict(path),
            _ => false,
        }
    }
}

impl Type for syn::Type {
    fn get_self(&self) -> &syn::Type {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn as_path_ok() {
        let r#type: syn::Type = syn_build::parse(quote! { std::option::Option<String> });
        assert!(matches!(r#type.as_path(), Ok(_)));
    }

    #[test]
    fn as_path_err() {
        let r#type: syn::Type = syn_build::parse(quote! { [u8; 32] });
        assert!(matches!(r#type.as_path(), Err(_)));
    }
}
