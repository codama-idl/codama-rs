use proc_macro2::TokenStream;
use quote::quote;

pub struct SynBuilder {}

impl SynBuilder {
    /// E.g. `{ pub foo: u32 }` for nammed or `(pub u32)` for unnamed.
    pub fn field(tt: TokenStream) -> syn::Field {
        let ast = syn::parse2::<syn::ItemStruct>(quote! { struct Foo #tt }).unwrap();
        let field = match &ast.fields {
            syn::Fields::Named(f) => f.named.first().cloned(),
            syn::Fields::Unnamed(f) => f.unnamed.first().cloned(),
            _ => None,
        };
        field.unwrap()
    }

    // TODO: try_field, etc. when extracting errors in separate module.
}
