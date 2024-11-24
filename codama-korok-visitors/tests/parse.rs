use codama_korok_visitors::{KorokVisitable, KorokVisitor};
use codama_koroks::{stores::RootStore, FieldKorok, RootKorok};
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

    struct MyVisitor {}
    impl KorokVisitor for MyVisitor {
        fn visit_field(&mut self, korok: &FieldKorok) {
            println!(
                "Field: {:#?}",
                korok.ast.ident.as_ref().unwrap().to_string()
            );
        }
    }

    let mut visitor = MyVisitor {};
    korok.accept(&mut visitor);
}
