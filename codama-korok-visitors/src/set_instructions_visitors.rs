use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{Attribute, Attributes, CodamaAttribute};
use codama_errors::CodamaResult;
use codama_koroks::FieldsKorok;
use codama_nodes::{
    EnumVariantTypeNode, InstructionAccountNode, InstructionArgumentNode, InstructionNode,
    NestedTypeNode, Node, ProgramNode, TypeNode,
};
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
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

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
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaInstructions` attribute.
        if !korok.attributes.has_codama_derive("CodamaInstructions") {
            return Ok(());
        };

        // Transform each variant into an `InstructionNode`.
        self.visit_children(korok)?;

        // Gather all instructions in a `ProgramNode`.
        let instructions = korok
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Instruction(instruction)) => Some(instruction.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        korok.node = Some(
            ProgramNode {
                instructions,
                ..ProgramNode::default()
            }
            .into(),
        );

        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Create a `EnumVariantNode` from the variant, if it doesn't already exist.
        self.combine_types.visit_enum_variant(korok)?;

        // Ensure we have a `Node`.
        let Some(node) = &korok.node else {
            return Err(korok
                .ast
                .error(format!("The \"{}\" TODO.", korok.ast.ident.to_string(),))
                .into());
        };

        // Ensure we have a `EnumStructVariantTypeNode`.
        let Ok(EnumVariantTypeNode::Struct(node)) = EnumVariantTypeNode::try_from(node.clone())
        else {
            return Err(korok
                .ast
                .error(format!("The \"{}\" TODO.", korok.ast.ident.to_string(),))
                .into());
        };

        // Ensure we have a non-nested `StructTypeNode`.
        let NestedTypeNode::Value(data) = node.r#struct else {
            return Err(korok
                .ast
                .error(format!("The \"{}\" TODO.", korok.ast.ident.to_string(),))
                .into());
        };

        let arguments: Vec<InstructionArgumentNode> = data.into();
        // TODO: set discriminator.
        // We need to keep track of the current discriminator index on the visitor and reset it when visiting a new enum.
        // We need to either increment it if no explicit discriminator exist on the variant, or set it to the explicit discriminator.
        // Then we can `.insert(0)` that discriminator into the arguments and set a `FieldDiscriminatorNode` on the instruction.
        // Right now, we'll call that field `discriminator` but we should offer a directive to customize it.

        korok.node = Some(
            InstructionNode {
                name: korok.ast.ident.to_string().into(),
                accounts: get_instruction_account_nodes(&korok.attributes, &korok.fields),
                arguments,
                ..InstructionNode::default()
            }
            .into(),
        );

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
