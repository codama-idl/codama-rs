use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{Node, NumberFormat::*, NumberTypeNode};
use quote::quote;

#[test]
fn it_identifies_usize_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(U64).into());
    assert_eq!(get_node_from_type(quote! { usize }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::usize }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::usize }), None);
    assert_eq!(get_node_from_type(quote! { usize<T> }), None);
}

#[test]
fn it_identifies_u8_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(U8).into());
    assert_eq!(get_node_from_type(quote! { u8 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u8 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u8 }), None);
    assert_eq!(get_node_from_type(quote! { u8<T> }), None);
}

#[test]
fn it_identifies_u16_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(U16).into());
    assert_eq!(get_node_from_type(quote! { u16 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u16 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u16 }), None);
    assert_eq!(get_node_from_type(quote! { u16<T> }), None);
}

#[test]
fn it_identifies_u32_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(U32).into());
    assert_eq!(get_node_from_type(quote! { u32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u32 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u32 }), None);
    assert_eq!(get_node_from_type(quote! { u32<T> }), None);
}

#[test]
fn it_identifies_u64_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(U64).into());
    assert_eq!(get_node_from_type(quote! { u64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u64 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::u64 }), None);
    assert_eq!(get_node_from_type(quote! { u64<T> }), None);
}

#[test]
fn it_identifies_u128_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(U128).into());
    assert_eq!(get_node_from_type(quote! { u128 }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::u128 }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::u128 }), None);
    assert_eq!(get_node_from_type(quote! { u128<T> }), None);
}

#[test]
fn it_identifies_isize_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(I64).into());
    assert_eq!(get_node_from_type(quote! { isize }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::isize }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::isize }), None);
    assert_eq!(get_node_from_type(quote! { isize<T> }), None);
}

#[test]
fn it_identifies_i8_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(I8).into());
    assert_eq!(get_node_from_type(quote! { i8 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i8 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i8 }), None);
    assert_eq!(get_node_from_type(quote! { i8<T> }), None);
}

#[test]
fn it_identifies_i16_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(I16).into());
    assert_eq!(get_node_from_type(quote! { i16 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i16 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i16 }), None);
    assert_eq!(get_node_from_type(quote! { i16<T> }), None);
}

#[test]
fn it_identifies_i32_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(I32).into());
    assert_eq!(get_node_from_type(quote! { i32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i32 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i32 }), None);
    assert_eq!(get_node_from_type(quote! { i32<T> }), None);
}

#[test]
fn it_identifies_i64_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(I64).into());
    assert_eq!(get_node_from_type(quote! { i64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i64 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::i64 }), None);
    assert_eq!(get_node_from_type(quote! { i64<T> }), None);
}

#[test]
fn it_identifies_i128_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(I128).into());
    assert_eq!(get_node_from_type(quote! { i128 }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::i128 }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::i128 }), None);
    assert_eq!(get_node_from_type(quote! { i128<T> }), None);
}

#[test]
fn it_identifies_f32_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(F32).into());
    assert_eq!(get_node_from_type(quote! { f32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::f32 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::f32 }), None);
    assert_eq!(get_node_from_type(quote! { f32<T> }), None);
}

#[test]
fn it_identifies_f64_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(F64).into());
    assert_eq!(get_node_from_type(quote! { f64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::f64 }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::f64 }), None);
    assert_eq!(get_node_from_type(quote! { f64<T> }), None);
}

#[test]
fn it_identifies_short_u16_numbers() {
    let expected: Option<Node> = Some(NumberTypeNode::le(ShortU16).into());
    assert_eq!(get_node_from_type(quote! { ShortU16 }), expected);
    assert_eq!(get_node_from_type(quote! { any::path::ShortU16 }), expected);
    assert_eq!(get_node_from_type(quote! { ShortU16<T> }), None);
}
