use codama_nodes::{
    BooleanTypeNode, Node, NumberFormat, NumberTypeNode, RegisteredTypeNode, SizePrefixTypeNode,
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
            let last_segment = path.segments.last().unwrap();
            let ident = &last_segment.ident;
            match ident.to_string().as_str() {
                "String" => Some(
                    SizePrefixTypeNode::new(
                        StringTypeNode::utf8(),
                        NumberTypeNode::le(NumberFormat::U32),
                    )
                    .into(),
                ),
                "bool" => Some(BooleanTypeNode::default().into()),
                "u8" => Some(NumberTypeNode::le(NumberFormat::U8).into()),
                "u16" => Some(NumberTypeNode::le(NumberFormat::U16).into()),
                "u32" => Some(NumberTypeNode::le(NumberFormat::U32).into()),
                "u64" => Some(NumberTypeNode::le(NumberFormat::U64).into()),
                "u128" => Some(NumberTypeNode::le(NumberFormat::U128).into()),
                "i8" => Some(NumberTypeNode::le(NumberFormat::I8).into()),
                "i16" => Some(NumberTypeNode::le(NumberFormat::I16).into()),
                "i32" => Some(NumberTypeNode::le(NumberFormat::I32).into()),
                "i64" => Some(NumberTypeNode::le(NumberFormat::I64).into()),
                "i128" => Some(NumberTypeNode::le(NumberFormat::I128).into()),
                "f32" => Some(NumberTypeNode::le(NumberFormat::F32).into()),
                "f64" => Some(NumberTypeNode::le(NumberFormat::F64).into()),
                "ShortU16" => Some(NumberTypeNode::le(NumberFormat::ShortU16).into()),
                _ => None,
            }
        }
        _ => None,
    }
}
