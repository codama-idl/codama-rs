use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{Node, NumberFormat::*, NumberTypeNode, RegisteredTypeNode};
use quote::quote;

#[test]
fn it_identifies_usize_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        U64,
    ))));
    assert_eq!(get_node_from_type(quote! { usize }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::usize }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::usize }), None);
}

#[test]
fn it_identifies_u8_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        U8,
    ))));
    assert_eq!(get_node_from_type(quote! { u8 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u8 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u8 }), None);
}

#[test]
fn it_identifies_u16_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        U16,
    ))));
    assert_eq!(get_node_from_type(quote! { u16 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u16 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u16 }), None);
}

#[test]
fn it_identifies_u32_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        U32,
    ))));
    assert_eq!(get_node_from_type(quote! { u32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u32 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u32 }), None);
}

#[test]
fn it_identifies_u64_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        U64,
    ))));
    assert_eq!(get_node_from_type(quote! { u64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u64 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u64 }), None);
}

#[test]
fn it_identifies_u128_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        U128,
    ))));
    assert_eq!(get_node_from_type(quote! { u128 }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::u128 }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::u128 }), None);
}

#[test]
fn it_identifies_isize_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        I64,
    ))));
    assert_eq!(get_node_from_type(quote! { isize }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::isize }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::isize }), None);
}

#[test]
fn it_identifies_i8_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        I8,
    ))));
    assert_eq!(get_node_from_type(quote! { i8 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i8 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i8 }), None);
}

#[test]
fn it_identifies_i16_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        I16,
    ))));
    assert_eq!(get_node_from_type(quote! { i16 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i16 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i16 }), None);
}

#[test]
fn it_identifies_i32_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        I32,
    ))));
    assert_eq!(get_node_from_type(quote! { i32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i32 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i32 }), None);
}

#[test]
fn it_identifies_i64_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        I64,
    ))));
    assert_eq!(get_node_from_type(quote! { i64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i64 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i64 }), None);
}

#[test]
fn it_identifies_i128_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        I128,
    ))));
    assert_eq!(get_node_from_type(quote! { i128 }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::i128 }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::i128 }), None);
}

#[test]
fn it_identifies_f32_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        F32,
    ))));
    assert_eq!(get_node_from_type(quote! { f32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::f32 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::f32 }), None);
}

#[test]
fn it_identifies_f64_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        F64,
    ))));
    assert_eq!(get_node_from_type(quote! { f64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::f64 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::f64 }), None);
}

#[test]
fn it_identifies_short_u16_numbers() {
    let expected = Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
        ShortU16,
    ))));
    assert_eq!(get_node_from_type(quote! { ShortU16 }), expected);
    assert_eq!(get_node_from_type(quote! { any::path::ShortU16 }), expected);
}
