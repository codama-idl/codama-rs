use crate::{
    walk_crate, walk_enum, walk_enum_variant, walk_field, walk_file_module, walk_module, walk_root,
    walk_struct, walk_type_alias, walk_unsupported_item, KorokVisitor,
};
use codama_attributes::{TryFromFilter, TypeDirective};
use codama_errors::CodamaResult;
use codama_koroks::{KorokMut, KorokTrait};
use codama_nodes::TypeNode;

#[derive(Default)]
pub struct ApplyTypeOverridesVisitor;

impl ApplyTypeOverridesVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ApplyTypeOverridesVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        walk_root(self, korok)?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        walk_crate(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        walk_file_module(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        walk_module(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        walk_struct(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        walk_enum(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        walk_unsupported_item(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        walk_enum_variant(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        walk_field(self, korok)?;
        apply_type_override(korok.into())
    }

    fn visit_type_alias(&mut self, korok: &mut codama_koroks::TypeAliasKorok) -> CodamaResult<()> {
        walk_type_alias(self, korok)?;
        apply_type_override(korok.into())
    }
}

fn apply_type_override(mut korok: KorokMut) -> CodamaResult<()> {
    let Some(attributes) = korok.attributes() else {
        return Ok(());
    };

    let Some(directive) = attributes.get_last(TypeDirective::filter) else {
        return Ok(());
    };

    let node = directive.node.try_resolved()?.clone();
    match (&mut korok, TypeNode::try_from(node.clone())) {
        (KorokMut::Field(field_korok), Ok(type_node)) => field_korok.set_type_node(type_node)?,
        _ => korok.set_node(Some(node.into())),
    };
    Ok(())
}
