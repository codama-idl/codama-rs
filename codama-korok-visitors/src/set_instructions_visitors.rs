use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{Attribute, Attributes, CodamaAttribute};
use codama_errors::CodamaResult;
use codama_koroks::FieldsKorok;
use codama_nodes::{InstructionAccountNode, InstructionNode, Node, TypeNode};
use codama_syn_helpers::extensions::ToTokensExtension;

pub struct SetInstructionsVisitor {
    combine_types: CombineTypesVisitor,
}

impl Default for SetInstructionsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor {
                // Skip fields with the `account` codama directive.
                get_nammed_field: |korok, parent| {
                    if korok.attributes.has_codama_attribute("account") {
                        return None;
                    }
                    CombineTypesVisitor::get_strict_nammed_field(korok, parent)
                },
                ..CombineTypesVisitor::strict()
            },
        }
    }
}

impl SetInstructionsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetInstructionsVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // Ensure the struct has the `CodamaInstruction` attribute.
        if !korok.attributes.has_codama_derive("CodamaInstruction") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the struct, if it doesn't already exist.
        self.combine_types.visit_struct(korok)?;

        // Ensure we have a `DefinedTypeNode` to work with.
        let Some(Node::DefinedType(defined_type)) = &korok.node else {
            return Err(korok
                .ast
                .error(format!(
                    "The \"{}\" struct could not be used as an Instruction because its type is not defined.",
                    korok.ast.ident.to_string(),
                ))
                .into());
        };

        // Ensure the data type is a struct.
        let TypeNode::Struct(data) = defined_type.r#type.clone() else {
            return Err(korok
                .ast
                .error(format!(
                    "The \"{}\" struct could not be used as an Instruction because its type is not a `StructTypeNode`.",
                    korok.ast.ident.to_string(),
                ))
                .into());
        };

        // Transform the defined type into an account node.
        korok.node = Some(
            InstructionNode {
                name: defined_type.name.clone(),
                docs: defined_type.docs.clone(),
                accounts: get_instruction_account_nodes(&korok.attributes, &korok.fields),
                arguments: data.into(),
                ..InstructionNode::default()
            }
            .into(),
        );

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;

        // Guard against `CodamaInstruction` enums.
        if korok.attributes.has_codama_derive("CodamaInstruction") {
            return Err(korok
                .ast
                .error(format!(
                    "The \"{}\" enum could not be used as an Instruction because the `CodamaInstruction` derive can only be used on structs. Did you mean to use `CodamaInstructions` instead?",
                    korok.ast.ident.to_string(),
                ))
                .into());
        };

        // TODO: Implement `CodamaInstructions`.
        Ok(())
    }
}

fn get_instruction_account_nodes(
    attributes: &Attributes,
    fields: &FieldsKorok,
) -> Vec<InstructionAccountNode> {
    // Gather the accounts from the struct attributes.
    let accounts_from_struct_attributes = attributes
        .iter()
        .filter_map(Attribute::codama)
        .filter_map(CodamaAttribute::account)
        .map(InstructionAccountNode::from)
        .collect::<Vec<_>>();

    // Gather the accounts from the fields.
    let accounts_from_fields = fields
        .all
        .iter()
        .filter_map(|field| {
            let account_attribute = field
                .attributes
                .iter()
                .filter_map(Attribute::codama)
                .filter_map(CodamaAttribute::account)
                .last();
            match account_attribute {
                Some(a) => Some(InstructionAccountNode::from(a)),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    // Concatenate the accounts.
    accounts_from_struct_attributes
        .into_iter()
        .chain(accounts_from_fields.into_iter())
        .collect::<Vec<_>>()
}
