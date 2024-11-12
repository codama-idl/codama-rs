use codama_koroks::{koroks::RootKorok, stores::RootStore};
use quote::quote;

fn main() {
    let tt = quote! {
        pub struct Command {
            pub executable: String,
            pub args: Vec<String>,
            pub env: Vec<String>,
            pub current_dir: String,
        }
    };

    let store = RootStore::populate_from(tt).unwrap();
    let korok = RootKorok::parse(&store).unwrap();

    eprintln!("{:#?}", korok);
}
