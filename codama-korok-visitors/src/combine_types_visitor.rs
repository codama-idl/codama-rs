use crate::KorokVisitor;
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_koroks::{EnumVariantKorok, FieldKorok};
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    EnumTypeNode, EnumVariantTypeNode, HasKind, Node, RegisteredTypeNode, StructFieldTypeNode,
    StructTypeNode, TupleTypeNode, TypeNode,
};
use codama_syn_helpers::extensions::*;

pub struct CombineTypesVisitor {
    pub r#override: bool,
    pub get_enum_variant: fn(
        korok: &EnumVariantKorok,
        parent: CombineTypesVisitorParent,
    ) -> Option<CodamaResult<EnumVariantTypeNode>>,
    pub get_nammed_field: fn(
        korok: &FieldKorok,
        parent: CombineTypesVisitorParent,
    ) -> Option<CodamaResult<StructFieldTypeNode>>,
    pub get_unnammed_field: fn(
        korok: &FieldKorok,
        parent: CombineTypesVisitorParent,
        index: usize,
    ) -> Option<CodamaResult<TypeNode>>,
    pub parent: CombineTypesVisitorParent,
}

impl Default for CombineTypesVisitor {
    fn default() -> Self {
        Self {
            r#override: false,
            get_enum_variant: |x, _| Self::get_default_enum_variant(x),
            get_nammed_field: |x, _| Self::get_default_nammed_field(x),
            get_unnammed_field: |x, _, _| Self::get_default_unnammed_field(x),
            parent: CombineTypesVisitorParent::None,
        }
    }
}

impl CombineTypesVisitor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn strict() -> Self {
        Self {
            get_enum_variant: Self::get_strict_enum_variant,
            get_nammed_field: Self::get_strict_nammed_field,
            get_unnammed_field: Self::get_strict_unnammed_field,
            ..Self::default()
        }
    }
    pub fn get_default_enum_variant(
        variant: &EnumVariantKorok,
    ) -> Option<CodamaResult<EnumVariantTypeNode>> {
        match &variant.node {
            Some(Node::Type(RegisteredTypeNode::EnumEmptyVariant(node))) => {
                Some(Ok(EnumVariantTypeNode::Empty(node.clone())))
            }
            Some(Node::Type(RegisteredTypeNode::EnumTupleVariant(node))) => {
                Some(Ok(EnumVariantTypeNode::Tuple(node.clone())))
            }
            Some(Node::Type(RegisteredTypeNode::EnumStructVariant(node))) => {
                Some(Ok(EnumVariantTypeNode::Struct(node.clone())))
            }
            _ => None,
        }
    }
    pub fn get_default_nammed_field(
        field: &FieldKorok,
    ) -> Option<CodamaResult<StructFieldTypeNode>> {
        match &field.node {
            Some(Node::Type(RegisteredTypeNode::StructField(field))) => Some(Ok(field.clone())),
            _ => None,
        }
    }
    pub fn get_default_unnammed_field(field: &FieldKorok) -> Option<CodamaResult<TypeNode>> {
        TypeNode::try_from(field.node.clone()).ok().map(Ok)
    }
    pub fn get_strict_enum_variant(
        variant: &EnumVariantKorok,
        parent: CombineTypesVisitorParent,
    ) -> Option<CodamaResult<EnumVariantTypeNode>> {
        match Self::get_default_enum_variant(variant) {
            Some(result) => Some(result),
            None => Some(Err(variant
                .ast
                .error(format!(
                    "Variant `{}` of {} does not resolve to a `EnumVariantTypeNode`",
                    variant.ast.ident.to_string(),
                    parent.identifier()
                ))
                .into())),
        }
    }
    pub fn get_strict_nammed_field(
        field: &FieldKorok,
        parent: CombineTypesVisitorParent,
    ) -> Option<CodamaResult<StructFieldTypeNode>> {
        match Self::get_default_nammed_field(field) {
            Some(result) => Some(result),
            None => Some(Err(field
                .ast
                .error(format!(
                    "Field `{}` in {} does not resolve to a `structFieldTypeNode`",
                    field.ast.ident.as_ref().unwrap().to_string(),
                    parent.identifier()
                ))
                .into())),
        }
    }
    pub fn get_strict_unnammed_field(
        field: &FieldKorok,
        parent: CombineTypesVisitorParent,
        index: usize,
    ) -> Option<CodamaResult<TypeNode>> {
        match Self::get_default_unnammed_field(field) {
            Some(result) => Some(result),
            None => Some(Err(field
                .ast
                .error(format!(
                    "Field `{}` in {} does not resolve to a `TypeNode`",
                    index.to_string(),
                    parent.identifier()
                ))
                .into())),
        }
    }
}

impl KorokVisitor for CombineTypesVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        self.parent = CombineTypesVisitorParent::Struct(korok.ast.ident.to_string());
        self.visit_children(korok)?;
        self.parent = CombineTypesVisitorParent::None;

        let name = korok.ast.ident.to_string();
        korok.node = match TypeNode::try_from(korok.fields.node.clone()) {
            Ok(TypeNode::Tuple(tuple_node)) if tuple_node.items.len() == 1 => {
                Some(DefinedTypeNode::new(name, tuple_node.items.first().unwrap().clone()).into())
            }
            Ok(type_node) => Some(DefinedTypeNode::new(name, type_node).into()),
            Err(_) => {
                let message = match &korok.fields.node {
                    Some(node) => format!(
                        "Cannot create a `definedTypeNode` from a node of kind `{}`",
                        node.kind()
                    ),
                    _ => "Cannot create a `definedTypeNode` from `None`".to_string(),
                };
                return Err(korok.ast.error(message).into());
            }
        };
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        let parent = CombineTypesVisitorParent::Enum(korok.ast.ident.to_string());
        self.parent = parent.clone();
        self.visit_children(korok)?;
        self.parent = CombineTypesVisitorParent::None;

        let enum_name = korok.ast.ident.to_string();
        let variants = korok
            .variants
            .iter()
            .filter_map(|variant| (self.get_enum_variant)(variant, parent.clone()))
            .collect_and_combine_errors()?;

        korok.node = Some(DefinedTypeNode::new(enum_name, EnumTypeNode::new(variants)).into());
        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        let original_parent_item = self.parent.clone();
        if let CombineTypesVisitorParent::Enum(name) = self.parent.clone() {
            self.parent = CombineTypesVisitorParent::Variant(name, korok.ast.ident.to_string());
        }
        self.visit_children(korok)?;
        self.parent = original_parent_item;

        let variant_name = korok.ast.ident.to_string();
        let discriminator = korok
            .ast
            .discriminant
            .as_ref()
            .and_then(|(_, x)| x.as_literal_integer::<usize>().ok());

        korok.node = match (&korok.ast.fields, &korok.fields.node) {
            (syn::Fields::Unit, _) => Some(
                EnumEmptyVariantTypeNode {
                    name: variant_name.into(),
                    discriminator,
                }
                .into(),
            ),
            (syn::Fields::Named(_), Some(Node::Type(RegisteredTypeNode::Struct(node)))) => Some(
                EnumStructVariantTypeNode {
                    name: variant_name.into(),
                    r#struct: node.clone().into(),
                    discriminator,
                }
                .into(),
            ),
            (syn::Fields::Unnamed(_), Some(Node::Type(RegisteredTypeNode::Tuple(node)))) => Some(
                EnumTupleVariantTypeNode {
                    name: variant_name.into(),
                    tuple: node.clone().into(),
                    discriminator,
                }
                .into(),
            ),
            (syn::Fields::Named(_), _) => {
                return Err(korok
                    .ast
                    .error(format!(
                        "Invalid node for enum variant `{}`. Expected a struct node.",
                        korok.ast.ident
                    ))
                    .into())
            }
            (syn::Fields::Unnamed(_), _) => {
                return Err(korok
                    .ast
                    .error(format!(
                        "Invalid node for enum variant `{}`. Expected a tuple node.",
                        korok.ast.ident
                    ))
                    .into())
            }
        };
        Ok(())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> CodamaResult<()> {
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        self.visit_children(korok)?;

        korok.node = match &korok.ast {
            syn::Fields::Named(_) => {
                let fields = korok
                    .all
                    .iter()
                    .filter_map(|field| (self.get_nammed_field)(field, self.parent.clone()))
                    .collect_and_combine_errors()?;
                Some(StructTypeNode::new(fields).into())
            }
            syn::Fields::Unnamed(_) => {
                let items = korok
                    .all
                    .iter()
                    .enumerate()
                    .filter_map(|(size, field)| {
                        (self.get_unnammed_field)(field, self.parent.clone(), size)
                    })
                    .collect_and_combine_errors()?;
                Some(TupleTypeNode::new(items).into())
            }
            syn::Fields::Unit => Some(StructTypeNode::new(vec![]).into()),
        };
        Ok(())
    }
}

#[derive(Default, Clone)]
pub enum CombineTypesVisitorParent {
    Struct(String),
    Enum(String),
    Variant(String, String),
    #[default]
    None,
}

impl CombineTypesVisitorParent {
    pub fn identifier(&self) -> String {
        match self {
            CombineTypesVisitorParent::Struct(name) => format!("struct `{}`", name),
            CombineTypesVisitorParent::Enum(name) => format!("enum `{}`", name),
            CombineTypesVisitorParent::Variant(enum_name, variant_name) => {
                format!("variant `{}` of enum `{}`", variant_name, enum_name)
            }
            CombineTypesVisitorParent::None => {
                unreachable!("This should only be called inside a parent item")
            }
        }
    }
}
