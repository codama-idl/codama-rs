use codama_korok_visitors::{CombineModulesVisitor, KorokVisitable};
use codama_koroks::ModuleKorok;
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumTypeNode, NumberTypeNode, ProgramNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, U32,
};
use codama_syn_helpers::syn_build;
use quote::quote;

#[test]
fn it_merges_types_into_program_nodes() {
    let ast = syn_build::parse(quote! {
        mod my_module {
            enum Membership { Free, Premium }
            struct User { name: String, age: u32 }
        }
    });
    let file_modules = Vec::new();
    let mut korok = ModuleKorok::parse(&ast, &file_modules, &mut 0).unwrap();
    let membership = DefinedTypeNode::new(
        "membership",
        EnumTypeNode::new(vec![
            EnumEmptyVariantTypeNode::new("free").into(),
            EnumEmptyVariantTypeNode::new("premium").into(),
        ]),
    );
    let person = DefinedTypeNode::new(
        "person",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
        ]),
    );
    korok.items[0].set_node(Some(membership.clone().into()));
    korok.items[1].set_node(Some(person.clone().into()));

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineModulesVisitor::new());
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                defined_types: vec![membership, person],
                ..Default::default()
            }
            .into()
        )
    );
}
