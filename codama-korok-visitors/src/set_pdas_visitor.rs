use crate::KorokVisitor;
use codama_attributes::{Attributes, SeedDirective, SeedDirectiveType, TryFromFilter};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    Docs, Node, PdaNode, PdaSeedNode, RegisteredTypeNode, TypeNode, VariablePdaSeedNode,
};

#[derive(Default)]
pub struct SetPdasVisitor;

impl SetPdasVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for SetPdasVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // Ensure the struct has the `CodamaPda` attribute.
        if !korok.attributes.has_codama_derive("CodamaPda") {
            return Ok(());
        };

        korok.node = Some(
            PdaNode {
                name: korok.name(),
                seeds: get_pda_seed_nodes(&korok.attributes, &korok.fields),
                docs: Docs::default(),
                program_id: None,
            }
            .into(),
        );
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // Ensure the enum has the `CodamaPda` attribute.
        if !korok.attributes.has_codama_derive("CodamaPda") {
            return Ok(());
        };

        korok.node = Some(
            PdaNode {
                name: korok.name(),
                seeds: get_pda_seed_nodes(&korok.attributes, &[]),
                docs: Docs::default(),
                program_id: None,
            }
            .into(),
        );
        Ok(())
    }
}

fn get_pda_seed_nodes(attributes: &Attributes, fields: &[FieldKorok]) -> Vec<PdaSeedNode> {
    attributes
        .iter()
        .filter_map(SeedDirective::filter)
        .filter_map(|directive| match &directive.seed {
            SeedDirectiveType::Defined(node) => Some(node.clone()),
            SeedDirectiveType::Linked(name) => fields.iter().find_map(|field| {
                if field.ast.ident.as_ref().is_none_or(|ident| ident != name) {
                    return None;
                }
                let (name, type_node) = match &field.node {
                    Some(Node::Type(RegisteredTypeNode::StructField(struct_field))) => {
                        (struct_field.name.clone(), struct_field.r#type.clone())
                    }
                    _ => match TypeNode::try_from(field.node.clone()) {
                        Ok(type_node) => (name.clone().into(), type_node),
                        Err(_) => return None,
                    },
                };
                Some(PdaSeedNode::Variable(VariablePdaSeedNode::new(
                    name, type_node,
                )))
            }),
        })
        .collect()
}
