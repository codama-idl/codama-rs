use codama_koroks::{stores::RootStore, RootKorok};
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
    println!("{:#?}", korok);
}
