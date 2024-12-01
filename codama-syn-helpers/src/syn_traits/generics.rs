use proc_macro2::TokenStream;
use quote::quote;

pub trait Generics {
    fn get_self(&self) -> &syn::Generics;

    fn param_idents(&self) -> TokenStream {
        let this = self.get_self();
        let idents = this
            .params
            .iter()
            .map(|param| match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    quote! { #ident }
                }
                syn::GenericParam::Lifetime(lifetime) => {
                    let lifetime = &lifetime.lifetime;
                    quote! { #lifetime }
                }
                syn::GenericParam::Const(const_param) => {
                    let ident = &const_param.ident;
                    quote! { #ident }
                }
            })
            .collect::<Vec<_>>();
        match idents.is_empty() {
            true => quote! {},
            false => quote! { <#(#idents),*> },
        }
    }

    // TODO: block_wrappers -> (TokenStream, TokenStream)
}

impl Generics for syn::Generics {
    fn get_self(&self) -> &syn::Generics {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn param_idents() {
        let r#struct: syn::ItemStruct = syn_build::parse(quote! { struct Foo(u32); });
        assert_eq!(
            r#struct.generics.param_idents().to_string(),
            quote! {}.to_string()
        );

        let r#struct: syn::ItemStruct =
            syn_build::parse(quote! { struct Foo<'a, T: Clone, U: PartialEq> where U: Eq (T); });
        assert_eq!(
            r#struct.generics.param_idents().to_string(),
            quote! { <'a, T, U> }.to_string()
        );
    }
}
