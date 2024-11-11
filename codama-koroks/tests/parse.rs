use quote::quote;
use syn::File;

fn main() {
    let foo = syn::parse2::<File>(quote! {
        pub struct Command {
            pub executable: String,
            pub args: Vec<String>,
            pub env: Vec<String>,
            pub current_dir: String,
        }
    });
    eprintln!("{:#?}", foo);
}
