use codama_korok_visitors::{BorshVisitor, KorokVisitable, KorokVisitor};
use codama_koroks::{stores::RootStore, FieldKorok, RootKorok, StructKorok};
use quote::quote;

fn main() {
    let tt = quote! {
        pub struct Command {
            pub executable: bool,
            pub code: u8,
            // pub args: Vec<String>,
            // pub env: Vec<String>,
            pub current_dir: String,
        }
    };

    let store = RootStore::populate_from(tt).unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();

    struct PrintVisitor {}
    impl KorokVisitor for PrintVisitor {
        fn visit_struct(&mut self, korok: &mut StructKorok) {
            let name = korok.ast.ident.to_string();
            println!("Struct {:#?}", name);
            println!("{:#?}", korok.node);

            for field_korok in &mut korok.fields {
                self.visit_field(field_korok);
            }
        }

        fn visit_field(&mut self, korok: &mut FieldKorok) {
            let name = korok.ast.ident.as_ref().unwrap().to_string();
            println!("Field {:#?}", name);
            println!("{:#?}", korok.node);
        }
    }

    let mut print_visitor = PrintVisitor {};
    let mut borsh_visitor = BorshVisitor {};
    korok.accept(&mut borsh_visitor);
    korok.accept(&mut print_visitor);
}
