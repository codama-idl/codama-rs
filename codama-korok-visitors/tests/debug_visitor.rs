use codama_errors::CodamaResult;
use codama_korok_visitors::{DebugVisitor, KorokVisitable, SetBorshTypesVisitor};
use codama_koroks::StructKorok;
use codama_nodes::{NumberFormat::U32, NumberTypeNode, StructFieldTypeNode};

#[test]
fn it_outputs_an_indented_debug_string() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo { bar: u32 } };
    let mut korok = StructKorok::parse(&item)?;
    korok.accept(&mut SetBorshTypesVisitor::new())?;

    let mut visitor = DebugVisitor::new();
    korok.accept(&mut visitor)?;

    let node_json =
        serde_json::to_string(&StructFieldTypeNode::new("bar", NumberTypeNode::le(U32)))?;

    assert_eq!(
        visitor.get_result(),
        format!(
            r#"Struct (Foo): null
|   Fields: null
|   |   Field (bar): {}
"#,
            node_json
        )
    );
    Ok(())
}
