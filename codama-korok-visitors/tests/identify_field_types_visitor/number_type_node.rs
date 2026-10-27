use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::{FloatFormat::*, FloatTypeNode, IntegerFormat::*, IntegerTypeNode, Node};
use quote::quote;

#[test]
fn it_identifies_usize_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(U64).into());
    assert_eq!(get_node_from_type(quote! { usize }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::usize }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { usize<T> }), None);
}

#[test]
fn it_identifies_u8_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(U8).into());
    assert_eq!(get_node_from_type(quote! { u8 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u8 }), expected);
    assert_eq!(get_node_from_type(quote! { u8<T> }), None);
}

#[test]
fn it_identifies_u16_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(U16).into());
    assert_eq!(get_node_from_type(quote! { u16 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u16 }), expected);
    assert_eq!(get_node_from_type(quote! { u16<T> }), None);
}

#[test]
fn it_identifies_u32_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(U32).into());
    assert_eq!(get_node_from_type(quote! { u32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u32 }), expected);
    assert_eq!(get_node_from_type(quote! { u32<T> }), None);
}

#[test]
fn it_identifies_u64_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(U64).into());
    assert_eq!(get_node_from_type(quote! { u64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::u64 }), expected);
    assert_eq!(get_node_from_type(quote! { u64<T> }), None);
}

#[test]
fn it_identifies_u128_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(U128).into());
    assert_eq!(get_node_from_type(quote! { u128 }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::u128 }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { u128<T> }), None);
}

#[test]
fn it_identifies_isize_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(I64).into());
    assert_eq!(get_node_from_type(quote! { isize }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::isize }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { isize<T> }), None);
}

#[test]
fn it_identifies_i8_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(I8).into());
    assert_eq!(get_node_from_type(quote! { i8 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i8 }), expected);
    assert_eq!(get_node_from_type(quote! { i8<T> }), None);
}

#[test]
fn it_identifies_i16_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(I16).into());
    assert_eq!(get_node_from_type(quote! { i16 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i16 }), expected);
    assert_eq!(get_node_from_type(quote! { i16<T> }), None);
}

#[test]
fn it_identifies_i32_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(I32).into());
    assert_eq!(get_node_from_type(quote! { i32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i32 }), expected);
    assert_eq!(get_node_from_type(quote! { i32<T> }), None);
}

#[test]
fn it_identifies_i64_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(I64).into());
    assert_eq!(get_node_from_type(quote! { i64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::i64 }), expected);
    assert_eq!(get_node_from_type(quote! { i64<T> }), None);
}

#[test]
fn it_identifies_i128_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(I128).into());
    assert_eq!(get_node_from_type(quote! { i128 }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::i128 }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { i128<T> }), None);
}

#[test]
fn it_identifies_f32_numbers() {
    let expected: Option<Node> = Some(FloatTypeNode::le(F32).into());
    assert_eq!(get_node_from_type(quote! { f32 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::f32 }), expected);
    assert_eq!(get_node_from_type(quote! { f32<T> }), None);
}

#[test]
fn it_identifies_f64_numbers() {
    let expected: Option<Node> = Some(FloatTypeNode::le(F64).into());
    assert_eq!(get_node_from_type(quote! { f64 }), expected);
    assert_eq!(get_node_from_type(quote! { std::primitive::f64 }), expected);
    assert_eq!(get_node_from_type(quote! { f64<T> }), None);
}

#[test]
fn it_identifies_short_u16_numbers() {
    let expected: Option<Node> = Some(IntegerTypeNode::le(ShortU16).into());
    assert_eq!(get_node_from_type(quote! { ShortU16 }), expected);
    assert_eq!(get_node_from_type(quote! { any::path::ShortU16 }), expected);
    assert_eq!(get_node_from_type(quote! { ShortU16<T> }), None);
}
