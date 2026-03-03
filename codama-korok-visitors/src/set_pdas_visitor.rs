use crate::KorokVisitor;
use codama_attributes::{
    Attributes, ProgramDirective, SeedDirective, SeedDirectiveType, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    CamelCaseString, Docs, Node, PdaNode, PdaSeedNode, ProgramNode, RegisteredTypeNode, TypeNode,
    VariablePdaSeedNode,
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

        let pda = parse_pda_node(korok.name(), &korok.attributes, &korok.fields);
        korok.node = Some(wrap_pda_in_program_node_when_program_directive_exists(
            pda,
            &korok.attributes,
        ));
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // Ensure the enum has the `CodamaPda` attribute.
        if !korok.attributes.has_codama_derive("CodamaPda") {
            return Ok(());
        };

        let pda = parse_pda_node(korok.name(), &korok.attributes, &[]);
        korok.node = Some(wrap_pda_in_program_node_when_program_directive_exists(
            pda,
            &korok.attributes,
        ));
        Ok(())
    }
}

pub fn parse_pda_node(
    name: CamelCaseString,
    attributes: &Attributes,
    fields: &[FieldKorok],
) -> PdaNode {
    PdaNode {
        name,
        seeds: parse_pda_seed_nodes(attributes, fields),
        docs: Docs::default(),
        program_id: None,
    }
}

pub fn parse_pda_seed_nodes(attributes: &Attributes, fields: &[FieldKorok]) -> Vec<PdaSeedNode> {
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

fn wrap_pda_in_program_node_when_program_directive_exists(
    pda: PdaNode,
    attributes: &Attributes,
) -> Node {
    let Some(program_directive) = attributes.get_last(ProgramDirective::filter) else {
        return pda.into();
    };

    ProgramNode {
        name: program_directive.name.clone().into(),
        public_key: program_directive.address.clone(),
        pdas: vec![pda],
        ..ProgramNode::default()
    }
    .into()
}
