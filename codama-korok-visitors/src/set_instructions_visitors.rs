use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{Attribute, Attributes, CodamaAttribute};
use codama_errors::CodamaResult;
use codama_koroks::FieldsKorok;
use codama_nodes::{
    DefaultValueStrategy, Docs, EnumVariantTypeNode, FieldDiscriminatorNode,
    InstructionAccountNode, InstructionArgumentNode, InstructionNode, NestedTypeNode, Node,
    NumberFormat::U8, NumberTypeNode, NumberValueNode, ProgramNode, TypeNode,
};
use codama_syn_helpers::extensions::{ExprExtension, ToTokensExtension};

pub struct SetInstructionsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: Option<String>,
    enum_current_discriminator: usize,
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
            enum_name: None,
            enum_current_discriminator: 0,
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
        self.enum_name = Some(korok.ast.ident.to_string());
        self.enum_current_discriminator = 0;
        self.visit_children(korok)?;
        self.enum_name = None;
        self.enum_current_discriminator = 0;

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
        // Update current discriminator.
        let current_discriminator = self.enum_current_discriminator;
        self.enum_current_discriminator = match &korok.ast.discriminant {
            Some((_, expr)) => expr.as_literal_integer()?,
            _ => current_discriminator + 1,
        };

        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Create a `EnumVariantNode` from the variant, if it doesn't already exist.
        self.combine_types.visit_enum_variant(korok)?;

        // Prepare the error message for wrong node types.
        let node_error_prefix = match &self.enum_name {
            Some(name) => format!(
                "The \"{}\" variant of the \"{}\" enum",
                korok.ast.ident, name
            ),
            None => format!("The \"{}\" variant", korok.ast.ident),
        };
        let node_error = korok.ast.error(format!(
            "{} could not be used as an Instruction because we cannot get a `StructTypeNode` for it. This is likely because it is not using nammed fields.",
            node_error_prefix
        ));

        // Ensure we have a `Node`.
        let Some(node) = &korok.node else {
            return Err(node_error.into());
        };

        // Ensure we have a `EnumStructVariantTypeNode`.
        let Ok(EnumVariantTypeNode::Struct(node)) = EnumVariantTypeNode::try_from(node.clone())
        else {
            return Err(node_error.into());
        };

        // Ensure we have a non-nested `StructTypeNode`.
        let NestedTypeNode::Value(data) = node.r#struct else {
            return Err(node_error.into());
        };

        let mut arguments: Vec<InstructionArgumentNode> = data.into();
        let discriminator_name = "discriminator".to_string(); // TODO: Offer a directive to customize this.
        let discriminator = InstructionArgumentNode {
            name: discriminator_name.clone().into(),
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            docs: Docs::default(),
            r#type: NumberTypeNode::le(U8).into(), // TODO: Get from enum type instead of assuming `u8`.
            default_value: Some(NumberValueNode::new(current_discriminator as u64).into()),
        };
        arguments.insert(0, discriminator);
        let discriminator_node = FieldDiscriminatorNode::new(discriminator_name, 0);

        korok.node = Some(
            InstructionNode {
                name: korok.ast.ident.to_string().into(),
                accounts: get_instruction_account_nodes(&korok.attributes, &korok.fields),
                arguments,
                discriminators: vec![discriminator_node.into()],
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
