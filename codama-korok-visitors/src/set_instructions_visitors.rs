use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{AccountDirective, Attributes, TryFromFilter};
use codama_errors::CodamaResult;
use codama_koroks::FieldsKorok;
use codama_nodes::{
    DefaultValueStrategy, DefinedTypeNode, Docs, EnumVariantTypeNode, FieldDiscriminatorNode,
    InstructionAccountNode, InstructionArgumentNode, InstructionNode, NestedTypeNode,
    NestedTypeNodeTrait, Node, NumberFormat::U8, NumberTypeNode, NumberValueNode, ProgramNode,
    StructTypeNode, TypeNode,
};
use codama_syn_helpers::extensions::{ExprExtension, ToTokensExtension};

pub struct SetInstructionsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: Option<String>,
    enum_size: Option<NumberTypeNode>,
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
            enum_size: None,
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

        // Transform the defined type into an instruction node.
        let data = get_struct_type_node_from_struct(korok)?;
        korok.node = Some(
            InstructionNode {
                name: korok.ast.ident.to_string().into(),
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

        // Create a `DefinedTypeNode` from the enum.
        self.combine_types.visit_enum(korok)?;

        // Get details from the defined type enum.
        let (enum_name, enum_size) = match &korok.node {
            Some(Node::DefinedType(DefinedTypeNode { name, r#type, .. })) => match r#type {
                TypeNode::Enum(data) => (
                    Some(name.to_string()),
                    Some(data.size.get_nested_type_node().clone()),
                ),
                _ => (Some(name.to_string()), None),
            },
            _ => (None, None),
        };

        // Transform each variant into an `InstructionNode`.
        self.enum_name = Some(enum_name.unwrap_or(korok.ast.ident.to_string()));
        self.enum_size = enum_size;
        self.enum_current_discriminator = 0;
        self.visit_children(korok)?;
        self.enum_name = None;
        self.enum_size = None;
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
        let current_discriminator = match &korok.ast.discriminant {
            Some((_, expr)) => expr.as_unsigned_integer()?,
            _ => self.enum_current_discriminator,
        };
        self.enum_current_discriminator = current_discriminator + 1;

        let data = get_struct_type_node_from_enum_variant(korok, &self.enum_name)?;
        let mut arguments: Vec<InstructionArgumentNode> = data.into();
        let discriminator_name = "discriminator".to_string(); // TODO: Offer a directive to customize this.
        let discriminator = InstructionArgumentNode {
            name: discriminator_name.clone().into(),
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            docs: Docs::default(),
            r#type: self
                .enum_size
                .clone()
                .unwrap_or(NumberTypeNode::le(U8))
                .into(),
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
        .filter_map(AccountDirective::filter)
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
                .filter_map(AccountDirective::filter)
                .last();
            account_attribute.map(InstructionAccountNode::from)
        })
        .collect::<Vec<_>>();

    // Concatenate the accounts.
    accounts_from_struct_attributes
        .into_iter()
        .chain(accounts_from_fields)
        .collect::<Vec<_>>()
}

fn get_struct_type_node_from_struct(
    korok: &codama_koroks::StructKorok,
) -> CodamaResult<StructTypeNode> {
    // Ensure we have a `DefinedTypeNode` to work with.
    if let Some(Node::DefinedType(node)) = &korok.node {
        // Ensure the data type is a struct.
        if let TypeNode::Struct(data) = node.r#type.clone() {
            return Ok(data);
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" struct could not be used as an Instruction because its type is not a `StructTypeNode`.",
        korok.ast.ident,
    );
    Err(korok.ast.error(message).into())
}

fn get_struct_type_node_from_enum_variant(
    korok: &codama_koroks::EnumVariantKorok,
    enum_name: &Option<String>,
) -> CodamaResult<StructTypeNode> {
    // Ensure we have a `Node`.
    if let Some(node) = &korok.node {
        // Ensure we have a `EnumVariantTypeNode`.
        if let Ok(node) = EnumVariantTypeNode::try_from(node.clone()) {
            match node {
                // Ensure we have a non-nested `StructTypeNode`.
                EnumVariantTypeNode::Struct(node) => {
                    if let NestedTypeNode::Value(data) = node.r#struct {
                        return Ok(data);
                    };
                }
                // Or an empty variant.
                EnumVariantTypeNode::Empty(_) => return Ok(StructTypeNode::new(vec![])),
                _ => {}
            }
        };
    };

    // Handle error.
    let message_prefix = match enum_name {
        Some(name) => format!(
            "The \"{}\" variant of the \"{}\" enum",
            korok.ast.ident, name
        ),
        None => format!("The \"{}\" variant", korok.ast.ident),
    };
    let message = format!(
        "{} could not be used as an Instruction because we cannot get a `StructTypeNode` for it. This is likely because it is not using nammed fields.",
        message_prefix
    );
    Err(korok.ast.error(message).into())
}
