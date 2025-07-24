use super::{PathExtension, ToTokensExtension};
use codama_errors::CodamaResult;
use syn::Type;

pub trait TypeExtension {
    fn get_self(&self) -> &Type;

    fn as_path(&self) -> CodamaResult<&syn::Path> {
        let this = self.get_self();
        match this {
            Type::Path(path) => Ok(&path.path),
            _ => Err(this.error("expected a path").into()),
        }
    }

    fn single_generic_type_from_path(&self, path: &str) -> CodamaResult<&Type> {
        let this = self.as_path()?;
        match this.is(path) {
            true => this.single_generic_type(),
            false => Err(this.error(format!("expected path: {path}")).into()),
        }
    }
}

impl TypeExtension for Type {
    fn get_self(&self) -> &Type {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_path_ok() {
        let r#type: Type = syn::parse_quote! { std::option::Option<String> };
        assert!(r#type.as_path().is_ok());
    }

    #[test]
    fn as_path_err() {
        let r#type: Type = syn::parse_quote! { [u8; 32] };
        assert!(r#type.as_path().is_err());
    }

    #[test]
    fn single_generic_type_from_path_ok() {
        let r#type: Type = syn::parse_quote! { std::option::Option<String> };
        assert!(r#type
            .single_generic_type_from_path("std::option::Option")
            .is_ok());

        let r#type: Type = syn::parse_quote! { Option<String> };
        assert!(r#type
            .single_generic_type_from_path("std::option::Option")
            .is_ok());
        assert!(r#type.single_generic_type_from_path("Option").is_ok());
    }

    #[test]
    fn single_generic_type_from_path_err() {
        let r#type: Type = syn::parse_quote! { [u8; 32] };
        assert!(r#type.single_generic_type_from_path("Option").is_err());

        let r#type: Type = syn::parse_quote! { std::option::Option<String> };
        assert!(r#type.single_generic_type_from_path("Option").is_err());
        assert!(r#type
            .single_generic_type_from_path("wrong::prefix::Option")
            .is_err());

        let r#type: Type = syn::parse_quote! { Option<String, u32> };
        assert!(r#type.single_generic_type_from_path("Option").is_err());
    }
}
