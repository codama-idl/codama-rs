use crate::{CombineTypesVisitor, KorokVisitor};
use codama_errors::CodamaResult;
use codama_nodes::{AccountNode, NestedTypeNode, Node, StructTypeNode};
use codama_syn_helpers::extensions::ToTokensExtension;

pub struct SetAccountsVisitor {
    combine_types: CombineTypesVisitor,
}

impl SetAccountsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SetAccountsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
        }
    }
}

impl KorokVisitor for SetAccountsVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // Ensure the struct has the `CodamaAccount` attribute.
        if !korok.attributes.has_codama_derive("CodamaAccount") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the struct, if it doesn't already exist.
        self.combine_types.visit_struct(korok)?;

        // Ensure we have a `DefinedTypeNode` to work with.
        let Some(Node::DefinedType(defined_type)) = &korok.node else {
            return Err(korok
                .ast
                .error(format!(
                    "The \"{}\" struct could not be used as an Account because its type is not defined.",
                    korok.ast.ident.to_string(),
                ))
                .into());
        };

        // Ensure the data type is a struct.
        let Ok(data) = NestedTypeNode::<StructTypeNode>::try_from(defined_type.r#type.clone())
        else {
            return Err(korok
                .ast
                .error(format!(
                    "The \"{}\" struct could not be used as an Account because its type is not a `NestedTypeNode<StructTypeNode>`.",
                    korok.ast.ident.to_string(),
                ))
                .into());
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

        // Guard against `CodamaAccount` enums.
        if korok.attributes.has_codama_derive("CodamaAccount") {
            return Err(korok
                .ast
                .error(format!(
                    "The \"{}\" enum could not be used as an Account because only structs are currently accepted.",
                    korok.ast.ident.to_string(),
                ))
                .into());
        };

        Ok(())
    }
}
