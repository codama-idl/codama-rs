use crate::KorokVisitor;
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
        self.visit_children(korok)?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_override(korok.into())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_override(korok.into())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_override(korok.into())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_override(korok.into())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_override(korok.into())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_override(korok.into())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_override(korok.into())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
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

    let registered_type_node = directive.node.clone();
    match (&mut korok, TypeNode::try_from(registered_type_node.clone())) {
        (KorokMut::Field(field_korok), Ok(type_node)) => field_korok.set_type_node(type_node),
        _ => korok.set_node(Some(registered_type_node.into())),
    };
    Ok(())
}
