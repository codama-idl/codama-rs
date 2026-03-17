use crate::KorokVisitor;
use codama_attributes::{
    Attributes, ProgramDirective, SeedDirective, SeedDirectiveType, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    CamelCaseString, ConstantPdaSeedNode, Docs, Node, PdaNode, PdaSeedNode, RegisteredTypeNode,
    TypeNode, VariablePdaSeedNode,
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

        let pda = parse_pda_node(korok.name(), &korok.attributes, &korok.fields)?;
        korok.node = Some(ProgramDirective::apply(&korok.attributes, pda.into()));
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // Ensure the enum has the `CodamaPda` attribute.
        if !korok.attributes.has_codama_derive("CodamaPda") {
            return Ok(());
        };

        let pda = parse_pda_node(korok.name(), &korok.attributes, &[])?;
        korok.node = Some(ProgramDirective::apply(&korok.attributes, pda.into()));
        Ok(())
    }
}

pub fn parse_pda_node(
    name: CamelCaseString,
    attributes: &Attributes,
    fields: &[FieldKorok],
) -> CodamaResult<PdaNode> {
    Ok(PdaNode {
        name,
        seeds: parse_pda_seed_nodes(attributes, fields)?,
        docs: Docs::default(),
        program_id: None,
    })
}

pub fn parse_pda_seed_nodes(
    attributes: &Attributes,
    fields: &[FieldKorok],
) -> CodamaResult<Vec<PdaSeedNode>> {
    let mut seeds = Vec::new();
    for directive in attributes.iter().filter_map(SeedDirective::filter) {
        match &directive.seed {
            SeedDirectiveType::Variable { name, r#type } => {
                let type_node = r#type.try_resolved()?.clone();
                seeds.push(VariablePdaSeedNode::new(name.as_str(), type_node).into());
            }
            SeedDirectiveType::Constant { r#type, value } => {
                let type_node = r#type.try_resolved()?.clone();
                let value_node = value.try_resolved()?.clone();
                seeds.push(ConstantPdaSeedNode::new(type_node, value_node).into());
            }
            SeedDirectiveType::Linked(name) => {
                // Linked seeds resolve by finding a matching field.
                // If no field matches, the seed is silently skipped (pre-existing behavior).
                let seed = fields.iter().find_map(|field| {
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
                });
                if let Some(seed) = seed {
                    seeds.push(seed);
                }
            }
        }
    }
    Ok(seeds)
}
