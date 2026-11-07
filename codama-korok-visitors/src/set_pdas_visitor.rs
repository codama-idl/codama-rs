use crate::KorokVisitor;
use codama_attributes::{
    Attributes, ProgramDirective, SeedDirective, SeedDirectiveType, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    ConstantPdaSeedNode, Docs, Identifier, Node, PdaNode, PdaSeedNode, RegisteredTypeNode,
    TypeNode, VariablePdaSeedNode,
};
use codama_syn_helpers::extensions::ToTokensExtension;

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

        let pda = parse_pda_node(korok.name()?, &korok.attributes, &korok.fields)?;
        korok.node = Some(ProgramDirective::apply(&korok.attributes, pda.into()));
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // Ensure the enum has the `CodamaPda` attribute.
        if !korok.attributes.has_codama_derive("CodamaPda") {
            return Ok(());
        };

        let pda = parse_pda_node(korok.name()?, &korok.attributes, &[])?;
        korok.node = Some(ProgramDirective::apply(&korok.attributes, pda.into()));
        Ok(())
    }
}

pub fn parse_pda_node(
    name: Identifier,
    attributes: &Attributes,
    fields: &[FieldKorok],
) -> CodamaResult<PdaNode> {
    Ok(PdaNode {
        identifier: name,
        seeds: parse_pda_seed_nodes(attributes, fields)?,
        docs: Docs::default(),
        program_id: None,
        plugins: vec![],
    })
}

pub fn parse_pda_seed_nodes(
    attributes: &Attributes,
    fields: &[FieldKorok],
) -> CodamaResult<Vec<PdaSeedNode>> {
    let mut seeds = Vec::new();
    let directives = attributes
        .iter()
        .filter_map(|attribute| Some((attribute, SeedDirective::filter(attribute)?)));
    for (attribute, directive) in directives {
        match &directive.seed {
            SeedDirectiveType::Variable { name, r#type } => {
                let type_node = r#type.try_resolved()?.clone();
                seeds.push(VariablePdaSeedNode::new(name.parse::<Identifier>()?, type_node).into());
            }
            SeedDirectiveType::Constant { r#type, value } => {
                let type_node = r#type.try_resolved()?.clone();
                let value_node = value.try_resolved()?.clone();
                seeds.push(ConstantPdaSeedNode::new(type_node, value_node).into());
            }
            SeedDirectiveType::Linked(name) => {
                let Some(field) = fields
                    .iter()
                    .find(|field| field.ast.ident.as_ref().is_some_and(|ident| ident == name))
                else {
                    let message = format!(
                        "Seed `{name}` links to a field that does not exist, \
                        add `type = ...` to the seed"
                    );
                    return Err(attribute.ast().error(message).into());
                };
                let (identifier, type_node) = match &field.node {
                    Some(Node::Type(RegisteredTypeNode::StructField(struct_field))) => (
                        struct_field.identifier.clone(),
                        (*struct_field.r#type).clone(),
                    ),
                    _ => match TypeNode::try_from(field.node.clone()) {
                        Ok(type_node) => {
                            let identifier =
                                Identifier::from_rust_name(name).map_err(|e| field.ast.error(e))?;
                            (identifier, type_node)
                        }
                        Err(_) => {
                            let message = format!(
                                "Seed `{name}` links to a field whose type does not resolve, \
                                add `type = ...` to the seed"
                            );
                            return Err(field.ast.error(message).into());
                        }
                    },
                };
                seeds.push(VariablePdaSeedNode::new(identifier, type_node).into());
            }
        }
    }
    Ok(seeds)
}
