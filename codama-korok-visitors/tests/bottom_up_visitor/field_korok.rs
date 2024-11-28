use crate::bottom_up_visitor::utils::{get_mock_syn_field, get_mock_syn_type};
use codama_korok_visitors::{BottomUpVisitor, KorokVisitable};
use codama_koroks::{FieldKorok, TypeKorok};
use codama_nodes::{NumberTypeNode, StructFieldTypeNode, U64};

#[test]
fn it_create_a_struct_field_type_node_when_nammed() {
    let mut korok = FieldKorok {
        ast: &get_mock_syn_field(Some(syn::Ident::new("foo", proc_macro2::Span::call_site()))),
        attributes: vec![],
        r#type: TypeKorok {
            ast: &get_mock_syn_type(),
            node: Some(NumberTypeNode::le(U64).into()),
        },
        node: None,
    };

    korok.accept(&mut BottomUpVisitor::new());

    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("foo", NumberTypeNode::le(U64)).into())
    );
}

#[test]
fn it_forwards_the_type_when_unnamed() {
    let mut korok = FieldKorok {
        ast: &get_mock_syn_field(None),
        attributes: vec![],
        r#type: TypeKorok {
            ast: &get_mock_syn_type(),
            node: Some(NumberTypeNode::le(U64).into()),
        },
        node: None,
    };

    korok.accept(&mut BottomUpVisitor::new());

    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
}
