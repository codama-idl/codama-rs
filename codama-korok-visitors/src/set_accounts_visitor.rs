use crate::{CombineTypesVisitor, KorokVisitor};
use codama_errors::CodamaResult;
use codama_nodes::{AccountNode, Docs, NestedTypeNode, Node, StructTypeNode};
use codama_syn_helpers::extensions::ToTokensExtension;

pub struct SetAccountsVisitor {
    combine_types: CombineTypesVisitor,
}

impl Default for SetAccountsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
        }
    }
}

impl SetAccountsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetAccountsVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaAccount` attribute.
        if !korok.attributes.has_codama_derive("CodamaAccount") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the struct, if it doesn't already exist.
        self.combine_types.visit_struct(korok)?;

        // Transform the defined type into an account node.
        let data = get_nested_struct_type_node_from_struct(korok)?;
        korok.node = Some(
            AccountNode {
                name: korok.ast.ident.to_string().into(),
                size: None,
                docs: Docs::default(),
                data,
                pda: None,
                discriminators: vec![],
            }
            .into(),
        );

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaAccounts` attribute.
        if !korok.attributes.has_codama_derive("CodamaAccounts") {
            return Ok(());
        };

        // TODO: Implements `CodamaAccounts` derive.

        Ok(())
    }
}

fn get_nested_struct_type_node_from_struct(
    korok: &codama_koroks::StructKorok,
) -> CodamaResult<NestedTypeNode<StructTypeNode>> {
    // Ensure we have a `DefinedTypeNode` to work with.
    if let Some(Node::DefinedType(node)) = &korok.node {
        // Ensure the data type is a struct.
        if let Ok(data) = NestedTypeNode::<StructTypeNode>::try_from(node.r#type.clone()) {
            return Ok(data);
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" struct could not be used as an Account because its type is not a `NestedTypeNode<StructTypeNode>`.",
        korok.ast.ident.to_string(),
    );
    Err(korok.ast.error(message).into())
}
