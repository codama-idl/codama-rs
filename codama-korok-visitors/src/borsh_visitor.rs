use codama_nodes::{
    BooleanTypeNode, Node, NumberFormat::*, NumberTypeNode, RegisteredTypeNode, SizePrefixTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNode,
};

use crate::KorokVisitor;

pub struct BorshVisitor {}

impl BorshVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl KorokVisitor for BorshVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        for field_korok in &mut korok.fields {
            self.visit_field(field_korok);
        }

        let is_all_struct_field = korok.fields.iter().all(|field| {
            matches!(
                field.node,
                Some(Node::Type(RegisteredTypeNode::StructField(_)))
            )
        });
        if is_all_struct_field {
            let fields = korok
                .fields
                .iter()
                .map(|field| {
                    if let Some(Node::Type(RegisteredTypeNode::StructField(field))) = &field.node {
                        field.clone()
                    } else {
                        panic!("Expected RegisteredTypeNode");
                    }
                })
                .collect::<Vec<_>>();
            let struct_node = StructTypeNode::new(fields);
            korok.node = Some(RegisteredTypeNode::Struct(struct_node).into());
            return ();
        }

        let is_all_tuple_item = korok.fields.iter().all(|field| {
            let Some(Node::Type(t)) = &field.node else {
                return false;
            };
            match t {
                RegisteredTypeNode::StructField(_) => false,
                RegisteredTypeNode::EnumEmptyVariant(_) => false,
                RegisteredTypeNode::EnumTupleVariant(_) => false,
                RegisteredTypeNode::EnumStructVariant(_) => false,
                _ => true,
            }
        });
        if is_all_tuple_item {
            let items = korok
                .fields
                .iter()
                .map(|field| {
                    let Some(Node::Type(t)) = &field.node else {
                        panic!("Expected RegisteredTypeNode");
                    };
                    TypeNode::try_from(t.clone()).unwrap()
                })
                .collect::<Vec<_>>();
            korok.node = Some(RegisteredTypeNode::Tuple(TupleTypeNode::new(items)).into());
            return ();
        }
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        let Some(node_type) = get_type_node_from_syn_type(&korok.ast.ty) else {
            return ();
        };

        match &korok.ast.ident {
            Some(ident) => {
                let field = StructFieldTypeNode::new(ident.to_string(), node_type);
                korok.node = Some(RegisteredTypeNode::StructField(field).into());
            }
            None => korok.node = Some(node_type.into()),
        };
    }
}

pub fn get_type_node_from_syn_type(ty: &syn::Type) -> Option<TypeNode> {
    match ty {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            if path.leading_colon.is_some() {
                return None;
            }
            let path_helper = PathHelper(path);
            match (
                // a::b::c::Option<T> -> a::b::c
                path_helper.prefix().as_str(),
                // a::b::c::Option<T> -> Option
                path_helper.last_indent().as_str(),
                // a::b::c::Option<T> -> T
                path_helper.generic_arguments().first_type(),
            ) {
                ("", "String", None) => Some(
                    SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into(),
                ),
                ("", "bool", None) => Some(BooleanTypeNode::default().into()),
                ("" | "std::primitive", "usize", None) => Some(NumberTypeNode::le(U64).into()),
                ("" | "std::primitive", "u8", None) => Some(NumberTypeNode::le(U8).into()),
                ("" | "std::primitive", "u16", None) => Some(NumberTypeNode::le(U16).into()),
                ("" | "std::primitive", "u32", None) => Some(NumberTypeNode::le(U32).into()),
                ("" | "std::primitive", "u64", None) => Some(NumberTypeNode::le(U64).into()),
                ("" | "std::primitive", "u128", None) => Some(NumberTypeNode::le(U128).into()),
                ("" | "std::primitive", "isize", None) => Some(NumberTypeNode::le(I64).into()),
                ("" | "std::primitive", "i8", None) => Some(NumberTypeNode::le(I8).into()),
                ("" | "std::primitive", "i16", None) => Some(NumberTypeNode::le(I16).into()),
                ("" | "std::primitive", "i32", None) => Some(NumberTypeNode::le(I32).into()),
                ("" | "std::primitive", "i64", None) => Some(NumberTypeNode::le(I64).into()),
                ("" | "std::primitive", "i128", None) => Some(NumberTypeNode::le(I128).into()),
                ("" | "std::primitive", "f32", None) => Some(NumberTypeNode::le(F32).into()),
                ("" | "std::primitive", "f64", None) => Some(NumberTypeNode::le(F64).into()),
                (_, "ShortU16", None) => Some(NumberTypeNode::le(ShortU16).into()),
                _ => None,
            }
        }
        _ => None,
    }
}

struct PathHelper<'a>(&'a syn::Path);

impl PathHelper<'_> {
    /// Returns all segment idents joined by "::" except the last one.
    /// E.g. for `a::b<B>::c::Option<T>` it returns `a::b::c`.
    pub fn prefix(&self) -> String {
        self.0
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()[..self.0.segments.len() - 1]
            .join("::")
    }

    /// Returns the last segment.
    pub fn last(&self) -> &syn::PathSegment {
        self.0.segments.last().unwrap()
    }

    /// Returns the ident of the last segment as a string.
    pub fn last_indent(&self) -> String {
        self.last().ident.to_string()
    }

    /// Returns the generic arguments of the last segment.
    /// E.g. for `a::b::c::Option<'a, T, U>` it returns `GenericArgumentsHelper(Some(['a, T, U]))`.
    /// E.g. for `a::b::c::u32` it returns `GenericArgumentsHelper(None)`.
    pub fn generic_arguments(&self) -> GenericArgumentsHelper {
        match &self.last().arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => GenericArgumentsHelper(Some(args)),
            _ => GenericArgumentsHelper(None),
        }
    }
}

struct GenericArgumentsHelper<'a>(
    Option<&'a syn::punctuated::Punctuated<syn::GenericArgument, syn::Token![,]>>,
);

impl GenericArgumentsHelper<'_> {
    /// Filters out all generic arguments that are not types.
    /// E.g. for `Option<'a, T, U>` it returns `[T, U]`.
    pub fn types(&self) -> Vec<&syn::Type> {
        match self.0 {
            Some(args) => args
                .iter()
                .filter_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                })
                .collect(),
            None => vec![],
        }
    }

    /// Returns the first genertic type argument if there is one.
    /// E.g. for `Vec<'a, T, U>` it returns `Some(T)`.
    pub fn first_type(&self) -> Option<&syn::Type> {
        self.types().first().copied()
    }
}
