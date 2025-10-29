use crate::KorokVisitor;
use codama_attributes::{Attributes, FieldDirective, ReprAttribute, TryFromFilter};
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_koroks::{EnumVariantKorok, FieldKorok, KorokTrait};
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    EnumTypeNode, EnumVariantTypeNode, NestedTypeNode, Node, NumberFormat::U8, NumberTypeNode,
    RegisteredTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNode,
};
use codama_syn_helpers::extensions::*;

pub struct CombineTypesVisitor {
    pub r#override: bool,
    pub get_enum_variant:
        fn(korok: &EnumVariantKorok, parent: &str) -> Option<CodamaResult<EnumVariantTypeNode>>,
    pub get_nammed_field:
        fn(korok: &FieldKorok, parent: &str) -> Option<CodamaResult<StructFieldTypeNode>>,
    pub get_unnammed_field:
        fn(korok: &FieldKorok, parent: &str, index: usize) -> Option<CodamaResult<TypeNode>>,
    pub parent_enum: String,
}

impl Default for CombineTypesVisitor {
    fn default() -> Self {
        Self {
            r#override: false,
            get_enum_variant: |x, _| Self::get_default_enum_variant(x),
            get_nammed_field: |x, _| Self::get_default_named_field(x),
            get_unnammed_field: |x, _, _| Self::get_default_unnamed_field(x),
            parent_enum: String::new(),
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
            get_nammed_field: Self::get_strict_named_field,
            get_unnammed_field: Self::get_strict_unnamed_field,
            ..Self::default()
        }
    }
    pub fn get_default_enum_variant(
        variant: &EnumVariantKorok,
    ) -> Option<CodamaResult<EnumVariantTypeNode>> {
        let Some(node) = &variant.node else {
            return None;
        };
        match EnumVariantTypeNode::try_from(node.clone()) {
            Ok(enum_variant_type_node) => Some(Ok(enum_variant_type_node)),
            _ => None,
        }
    }
    pub fn get_default_named_field(
        field: &FieldKorok,
    ) -> Option<CodamaResult<StructFieldTypeNode>> {
        match &field.node {
            Some(Node::Type(RegisteredTypeNode::StructField(field))) => Some(Ok(field.clone())),
            _ => None,
        }
    }
    pub fn get_default_unnamed_field(field: &FieldKorok) -> Option<CodamaResult<TypeNode>> {
        TypeNode::try_from(field.node.clone()).ok().map(Ok)
    }
    pub fn get_strict_enum_variant(
        variant: &EnumVariantKorok,
        parent: &str,
    ) -> Option<CodamaResult<EnumVariantTypeNode>> {
        match Self::get_default_enum_variant(variant) {
            Some(result) => Some(result),
            None => Some(Err(variant
                .ast
                .error(format!(
                    "Variant `{}` of {} does not resolve to a `EnumVariantTypeNode`",
                    variant.ast.ident, parent
                ))
                .into())),
        }
    }
    pub fn get_strict_named_field(
        field: &FieldKorok,
        parent: &str,
    ) -> Option<CodamaResult<StructFieldTypeNode>> {
        match Self::get_default_named_field(field) {
            Some(result) => Some(result),
            None => Some(Err(field
                .ast
                .error(format!(
                    "Field `{}` in {} does not resolve to a `structFieldTypeNode`",
                    field.ast.ident.as_ref().unwrap(),
                    parent
                ))
                .into())),
        }
    }
    pub fn get_strict_unnamed_field(
        field: &FieldKorok,
        parent: &str,
        index: usize,
    ) -> Option<CodamaResult<TypeNode>> {
        match Self::get_default_unnamed_field(field) {
            Some(result) => Some(result),
            None => Some(Err(field
                .ast
                .error(format!(
                    "Field `{}` in {} does not resolve to a `TypeNode`",
                    index, parent
                ))
                .into())),
        }
    }

    fn parse_named_fields(
        &self,
        fields: &[FieldKorok],
        attributes: &Attributes,
        parent: &str,
    ) -> CodamaResult<Vec<StructFieldTypeNode>> {
        let fields = fields
            .iter()
            .filter_map(|field| (self.get_nammed_field)(field, parent))
            .collect_and_combine_errors()?;

        let (before, after): (Vec<_>, Vec<_>) = attributes
            .get_all(FieldDirective::filter)
            .into_iter()
            .partition(|attr| !attr.after);
        let before = before.into_iter().map(|attr| attr.field.clone());
        let after = after.into_iter().map(|attr| attr.field.clone());

        Ok(before.into_iter().chain(fields).chain(after).collect())
    }

    fn parse_unnamed_fields(
        &self,
        fields: &[FieldKorok],
        parent: &str,
    ) -> CodamaResult<Vec<TypeNode>> {
        let items = fields
            .iter()
            .enumerate()
            .filter_map(|(index, field)| (self.get_unnammed_field)(field, parent, index))
            .collect_and_combine_errors()?;

        Ok(items)
    }
}

impl KorokVisitor for CombineTypesVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        self.visit_children(korok)?;

        let parent = format!("struct `{}`", korok.ast.ident);
        let type_node: TypeNode = match korok.ast.fields {
            syn::Fields::Named(_) => {
                let fields = self.parse_named_fields(&korok.fields, &korok.attributes, &parent)?;
                StructTypeNode::new(fields).into()
            }
            syn::Fields::Unnamed(_) => {
                let items = self.parse_unnamed_fields(&korok.fields, &parent)?;
                if items.len() == 1 {
                    items.first().unwrap().clone()
                } else {
                    TupleTypeNode::new(items).into()
                }
            }
            _ => {
                let fields = self.parse_named_fields(&korok.fields, &korok.attributes, &parent)?;
                StructTypeNode::new(fields).into()
            }
        };

        korok.set_node(Some(DefinedTypeNode::new(korok.name(), type_node).into()));
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        self.parent_enum = korok.ast.ident.to_string();
        self.visit_children(korok)?;
        self.parent_enum.clear();

        let parent = format!("enum `{}`", korok.ast.ident);
        let variants = korok
            .variants
            .iter()
            .filter_map(|variant| (self.get_enum_variant)(variant, &parent))
            .collect_and_combine_errors()?;

        let size = korok
            .attributes
            .get_first(ReprAttribute::filter)
            .and_then(|attr| attr.get_number_type_node())
            .unwrap_or(NumberTypeNode::le(U8));

        korok.node = Some(
            DefinedTypeNode::new(
                korok.name(),
                EnumTypeNode {
                    variants,
                    size: NestedTypeNode::Value(size),
                },
            )
            .into(),
        );
        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        self.visit_children(korok)?;

        let parent = format!(
            "variant `{}` of enum `{}`",
            korok.ast.ident, self.parent_enum
        );
        let discriminator = korok
            .ast
            .discriminant
            .as_ref()
            .and_then(|(_, x)| x.as_unsigned_integer::<usize>().ok());

        korok.node = match korok.ast.fields {
            syn::Fields::Named(_) => {
                let fields = self.parse_named_fields(&korok.fields, &korok.attributes, &parent)?;
                Some(
                    EnumStructVariantTypeNode {
                        name: korok.name(),
                        r#struct: StructTypeNode::new(fields).into(),
                        discriminator,
                    }
                    .into(),
                )
            }
            syn::Fields::Unnamed(_) => {
                let items = self.parse_unnamed_fields(&korok.fields, &parent)?;
                Some(
                    EnumTupleVariantTypeNode {
                        name: korok.name(),
                        tuple: TupleTypeNode::new(items).into(),
                        discriminator,
                    }
                    .into(),
                )
            }
            _ => {
                let fields = self.parse_named_fields(&korok.fields, &korok.attributes, &parent)?;
                if !fields.is_empty() {
                    Some(
                        EnumStructVariantTypeNode {
                            name: korok.name(),
                            r#struct: StructTypeNode::new(fields).into(),
                            discriminator,
                        }
                        .into(),
                    )
                } else {
                    Some(
                        EnumEmptyVariantTypeNode {
                            name: korok.name(),
                            discriminator,
                        }
                        .into(),
                    )
                }
            }
        };
        Ok(())
    }
}
