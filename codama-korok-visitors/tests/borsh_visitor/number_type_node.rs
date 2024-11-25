use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{Node, NumberFormat, NumberTypeNode, RegisteredTypeNode};
use quote::quote;

#[test]
fn it_identifies_usize_numbers() {
    assert_eq!(
        get_node_from_type(quote! { usize }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::U64
        ))))
    );
}

#[test]
fn it_identifies_u8_numbers() {
    assert_eq!(
        get_node_from_type(quote! { u8 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::U8
        ))))
    );
}

#[test]
fn it_identifies_u16_numbers() {
    assert_eq!(
        get_node_from_type(quote! { u16 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::U16
        ))))
    );
}

#[test]
fn it_identifies_u32_numbers() {
    assert_eq!(
        get_node_from_type(quote! { u32 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::U32
        ))))
    );
}

#[test]
fn it_identifies_u64_numbers() {
    assert_eq!(
        get_node_from_type(quote! { u64 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::U64
        ))))
    );
}

#[test]
fn it_identifies_u128_numbers() {
    assert_eq!(
        get_node_from_type(quote! { u128 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::U128
        ))))
    );
}

#[test]
fn it_identifies_isize_numbers() {
    assert_eq!(
        get_node_from_type(quote! { isize }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::I64
        ))))
    );
}

#[test]
fn it_identifies_i8_numbers() {
    assert_eq!(
        get_node_from_type(quote! { i8 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::I8
        ))))
    );
}

#[test]
fn it_identifies_i16_numbers() {
    assert_eq!(
        get_node_from_type(quote! { i16 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::I16
        ))))
    );
}

#[test]
fn it_identifies_i32_numbers() {
    assert_eq!(
        get_node_from_type(quote! { i32 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::I32
        ))))
    );
}

#[test]
fn it_identifies_i64_numbers() {
    assert_eq!(
        get_node_from_type(quote! { i64 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::I64
        ))))
    );
}

#[test]
fn it_identifies_i128_numbers() {
    assert_eq!(
        get_node_from_type(quote! { i128 }),
        Some(Node::Type(RegisteredTypeNode::Number(NumberTypeNode::le(
            NumberFormat::I128
        ))))
    );
}
