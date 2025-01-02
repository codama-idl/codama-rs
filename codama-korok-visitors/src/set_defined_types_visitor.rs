use crate::{CombineTypesVisitor, KorokVisitor};
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_syn_helpers::extensions::ToTokensExtension;

#[derive(Default)]
pub struct SetDefinedTypesVisitor {
    combine_types: CombineTypesVisitor,
    parent_item: ParentItem,
}

#[derive(Default, Clone)]
enum ParentItem {
    Struct(String),
    Enum(String),
    Variant(String, String),
    #[default]
    None,
}

impl SetDefinedTypesVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ParentItem {
    fn identifier(&self) -> String {
        match self {
            ParentItem::Struct(name) => format!("struct `{}`", name),
            ParentItem::Enum(name) => format!("enum `{}`", name),
            ParentItem::Variant(enum_name, variant_name) => {
                format!("variant `{}` of enum `{}`", variant_name, enum_name)
            }
            ParentItem::None => {
                unreachable!("This should only be called inside a struct or enum item")
            }
        }
    }
}

impl KorokVisitor for SetDefinedTypesVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // Ensure the struct has the `CodamaType` attribute.
        if !korok.attributes.has_codama_derive("CodamaType") {
            return Ok(());
        };

        // Create a DefinedTypeNode from the struct.
        self.combine_types.visit_struct(korok)?;

        // Ensure all fields are defined.
        self.parent_item = ParentItem::Struct(korok.ast.ident.to_string());
        self.visit_children(korok)?;
        self.parent_item = ParentItem::None;
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // Ensure the enum has the `CodamaType` attribute.
        if !korok.attributes.has_codama_derive("CodamaType") {
            return Ok(());
        };

        // Create a DefinedTypeNode from the enum.
        self.combine_types.visit_enum(korok)?;

        // Ensure all variants are defined.
        self.parent_item = ParentItem::Enum(korok.ast.ident.to_string());
        self.visit_children(korok)?;
        self.parent_item = ParentItem::None;
        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        let original_parent_item = self.parent_item.clone();
        if let ParentItem::Enum(name) = self.parent_item.clone() {
            self.parent_item = ParentItem::Variant(name, korok.ast.ident.to_string());
        }
        self.visit_children(korok)?;
        self.parent_item = original_parent_item;
        Ok(())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> CodamaResult<()> {
        korok
            .all
            .iter()
            .enumerate()
            .map(|(index, field)| -> CodamaResult<()> {
                if !CombineTypesVisitor::is_valid_field(field) {
                    let message = match &field.ast.ident {
                        Some(ident) => format!(
                            "Field `{}` in {} does not resolve to a `structFieldTypeNode`",
                            ident.to_string(),
                            self.parent_item.identifier()
                        ),
                        None => format!(
                            "Field `{}` in {} does not resolve to a `TypeNode`",
                            index.to_string(),
                            self.parent_item.identifier()
                        ),
                    };
                    return Err(field.ast.error(message).into());
                }
                Ok(())
            })
            .collect_and_combine_errors()?;
        Ok(())
    }
}
