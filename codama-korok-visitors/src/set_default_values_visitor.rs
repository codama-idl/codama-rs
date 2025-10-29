use crate::KorokVisitor;
use codama_attributes::{DefaultValueDirective, TryFromFilter};
use codama_errors::CodamaResult;
use codama_koroks::{KorokMut, KorokTrait};
use codama_nodes::{
    InstructionArgumentNode, Node, RegisteredTypeNode, StructFieldTypeNode, ValueNode,
};

#[derive(Default)]
pub struct SetDefaultValuesVisitor;

impl SetDefaultValuesVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for SetDefaultValuesVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        set_default_values(korok.into())
    }
}

fn set_default_values(mut korok: KorokMut) -> CodamaResult<()> {
    // Ensure it has attributes.
    let Some(attributes) = korok.attributes() else {
        return Ok(());
    };

    // Ensure it has a default value directive.
    let Some(directive) = attributes.get_last(DefaultValueDirective::filter) else {
        return Ok(());
    };

    let node = match korok.node() {
        // Handle struct fields.
        Some(Node::Type(RegisteredTypeNode::StructField(field))) => {
            match ValueNode::try_from(directive.node.clone()) {
                Ok(value) => Some(
                    StructFieldTypeNode {
                        default_value: Some(value),
                        ..field.clone()
                    }
                    .into(),
                ),
                _ => return Ok(()),
            }
        }
        // Handle instruction arguments.
        Some(Node::InstructionArgument(argument)) => Some(
            InstructionArgumentNode {
                default_value: Some(directive.node.clone()),
                ..argument.clone()
            }
            .into(),
        ),
        _ => return Ok(()),
    };

    korok.set_node(node);
    Ok(())
}
