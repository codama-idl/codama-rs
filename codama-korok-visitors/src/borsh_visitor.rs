use codama_nodes::{
    ArrayTypeNode, BooleanTypeNode, FixedCountNode, MapTypeNode, NumberFormat::*, NumberTypeNode,
    PrefixedCountNode, PublicKeyTypeNode, SetTypeNode, SizePrefixTypeNode, StringTypeNode,
    TypeNode,
};
use codama_syn_helpers::syn_wrap;

use crate::KorokVisitor;

pub struct BorshVisitor {}

impl BorshVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl KorokVisitor for BorshVisitor {
    fn visit_type(&mut self, korok: &mut codama_koroks::TypeKorok) {
        korok.node = match self.get_type_node(&korok.ast) {
            Some(node) => Some(node.into()),
            None => None,
        };
    }
}

impl BorshVisitor {
    fn get_type_node(&self, ty: &syn::Type) -> Option<TypeNode> {
        match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                if path.leading_colon.is_some() {
                    return None;
                }
                let path_helper = syn_wrap::Path(path);
                match (
                    // a::b<B>::c::HashMap<K, V> -> a::b::c
                    path_helper.prefix().as_str(),
                    // a::b::c::HashMap<K, V> -> HashMap
                    path_helper.last_indent().as_str(),
                    // a::b::c::HashMap<K, V> -> [K, V]
                    path_helper.generic_types().as_slice(),
                ) {
                    ("" | "std::primitive", "bool", []) => Some(BooleanTypeNode::default().into()),
                    ("" | "std::primitive", "usize", []) => Some(NumberTypeNode::le(U64).into()),
                    ("" | "std::primitive", "u8", []) => Some(NumberTypeNode::le(U8).into()),
                    ("" | "std::primitive", "u16", []) => Some(NumberTypeNode::le(U16).into()),
                    ("" | "std::primitive", "u32", []) => Some(NumberTypeNode::le(U32).into()),
                    ("" | "std::primitive", "u64", []) => Some(NumberTypeNode::le(U64).into()),
                    ("" | "std::primitive", "u128", []) => Some(NumberTypeNode::le(U128).into()),
                    ("" | "std::primitive", "isize", []) => Some(NumberTypeNode::le(I64).into()),
                    ("" | "std::primitive", "i8", []) => Some(NumberTypeNode::le(I8).into()),
                    ("" | "std::primitive", "i16", []) => Some(NumberTypeNode::le(I16).into()),
                    ("" | "std::primitive", "i32", []) => Some(NumberTypeNode::le(I32).into()),
                    ("" | "std::primitive", "i64", []) => Some(NumberTypeNode::le(I64).into()),
                    ("" | "std::primitive", "i128", []) => Some(NumberTypeNode::le(I128).into()),
                    ("" | "std::primitive", "f32", []) => Some(NumberTypeNode::le(F32).into()),
                    ("" | "std::primitive", "f64", []) => Some(NumberTypeNode::le(F64).into()),
                    (_, "ShortU16", []) => Some(NumberTypeNode::le(ShortU16).into()),
                    (
                        "" | "solana_sdk::pubkey" | "solana_program" | "solana_pubkey",
                        "Pubkey",
                        [],
                    ) => Some(PublicKeyTypeNode::new().into()),
                    ("" | "std::string", "String", []) => Some(
                        SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32))
                            .into(),
                    ),
                    ("" | "std::vec", "Vec", [t]) => match self.get_type_node(t) {
                        Some(item) => Some(
                            ArrayTypeNode::new(
                                item,
                                PrefixedCountNode::new(NumberTypeNode::le(U32)),
                            )
                            .into(),
                        ),
                        None => None,
                    },
                    ("" | "std::collections", "HashSet" | "BTreeSet", [t]) => {
                        match self.get_type_node(t) {
                            Some(item) => Some(
                                SetTypeNode::new(
                                    item,
                                    PrefixedCountNode::new(NumberTypeNode::le(U32)),
                                )
                                .into(),
                            ),
                            None => None,
                        }
                    }
                    ("" | "std::collections", "HashMap" | "BTreeMap", [k, v]) => {
                        match (self.get_type_node(k), self.get_type_node(v)) {
                            (Some(key), Some(value)) => Some(
                                MapTypeNode::new(
                                    key,
                                    value,
                                    PrefixedCountNode::new(NumberTypeNode::le(U32)),
                                )
                                .into(),
                            ),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            syn::Type::Array(syn::TypeArray { elem, len, .. }) => {
                let Ok(size) = syn_wrap::Expr(len).as_literal_integer::<usize>() else {
                    return None;
                };
                match self.get_type_node(elem) {
                    Some(item) => Some(ArrayTypeNode::new(item, FixedCountNode::new(size)).into()),
                    None => None,
                }
            }
            _ => None,
        }
    }
}
