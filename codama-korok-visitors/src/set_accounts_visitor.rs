use crate::KorokVisitor;
use codama_errors::CodamaResult;
use codama_nodes::{AccountNode, NestedTypeNode, Node, StructTypeNode};

#[derive(Default)]
pub struct SetAccountsVisitor {}

impl SetAccountsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetAccountsVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;

        // Ensure the struct has the `CodamaAccount` attribute.
        if !korok.attributes.has_codama_derive("CodamaAccount") {
            return Ok(());
        };

        // Ensure the korok is already typed.
        let Some(Node::DefinedType(defined_type)) = &korok.node else {
            // TODO: Throw error?
            return Ok(());
        };

        // Ensure the data type is a struct.
        let Ok(data) = NestedTypeNode::<StructTypeNode>::try_from(defined_type.r#type.clone())
        else {
            // TODO: Throw error?
            return Ok(());
        };

        // Transform the defined type into an account node.
        korok.node = Some(
            AccountNode {
                name: defined_type.name.clone(),
                size: None,
                docs: defined_type.docs.clone(),
                data,
                pda: None,
                discriminators: vec![],
            }
            .into(),
        );

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;

        // Ensure the struct has the `CodamaAccount` attribute.
        if !korok.attributes.has_codama_derive("CodamaAccount") {
            return Ok(());
        };

        // TODO: Throw error?
        korok.node = None;
        Ok(())
    }
}
