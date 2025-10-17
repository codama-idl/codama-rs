use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{
    AccountDirective, ArgumentDirective, Attributes, DiscriminatorDirective,
    EnumDiscriminatorDirective, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, EnumVariantTypeNode, FieldDiscriminatorNode,
    InstructionAccountNode, InstructionArgumentNode, InstructionNode, NestedTypeNode, Node,
    NumberValueNode, ProgramNode, StructTypeNode, TypeNode,
};
use codama_syn_helpers::extensions::{ExprExtension, ToTokensExtension};

pub struct SetInstructionsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: String,
    enum_discriminator: EnumDiscriminatorDirective,
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
                    CombineTypesVisitor::get_strict_named_field(korok, parent)
                },
                ..CombineTypesVisitor::strict()
            },
            enum_name: "".to_string(),
            enum_discriminator: EnumDiscriminatorDirective::default(),
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
        let (name, data) = parse_struct(korok)?;
        korok.node = Some(
            InstructionNode {
                name,
                accounts: parse_accounts(&korok.attributes, &korok.fields),
                arguments: parse_arguments(&korok.attributes, data, None),
                discriminators: DiscriminatorDirective::nodes(&korok.attributes),
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

        // Get enum discriminator info.
        let enum_discriminator = korok
            .attributes
            .get_last(EnumDiscriminatorDirective::filter)
            .cloned()
            .unwrap_or_else(|| EnumDiscriminatorDirective::from(&korok.node));

        // Transform each variant into an `InstructionNode`.
        self.enum_name = korok.ast.ident.to_string();
        self.enum_discriminator = enum_discriminator;
        self.enum_current_discriminator = 0;
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
        // Update current discriminator.
        let current_discriminator = match &korok.ast.discriminant {
            Some((_, expr)) => expr.as_unsigned_integer()?,
            _ => self.enum_current_discriminator,
        };
        self.enum_current_discriminator = current_discriminator + 1;

        let (name, data) = parse_enum_variant(korok, &self.enum_name)?;
        let discriminator = InstructionArgumentNode {
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            default_value: Some(NumberValueNode::new(current_discriminator as u64).into()),
            ..InstructionArgumentNode::from(&self.enum_discriminator)
        };
        let mut discriminators = DiscriminatorDirective::nodes(&korok.attributes);
        discriminators.insert(
            0,
            FieldDiscriminatorNode::new(discriminator.name.clone(), 0).into(),
        );

        korok.node = Some(
            InstructionNode {
                name,
                accounts: parse_accounts(&korok.attributes, &korok.fields),
                arguments: parse_arguments(&korok.attributes, data, Some(discriminator)),
                discriminators,
                ..InstructionNode::default()
            }
            .into(),
        );

        Ok(())
    }
}

fn parse_accounts(attributes: &Attributes, fields: &[FieldKorok]) -> Vec<InstructionAccountNode> {
    // Gather the accounts from the struct attributes.
    let accounts_from_struct_attributes = attributes
        .iter()
        .filter_map(AccountDirective::filter)
        .map(|attr| attr.account.clone())
        .collect::<Vec<_>>();

    // Gather the accounts from the fields.
    let accounts_from_fields = fields
        .iter()
        .filter_map(|field| {
            field
                .attributes
                .get_last(AccountDirective::filter)
                .map(|attr| attr.account.clone())
        })
        .collect::<Vec<_>>();

    // Concatenate the accounts.
    accounts_from_struct_attributes
        .into_iter()
        .chain(accounts_from_fields)
        .collect::<Vec<_>>()
}

fn parse_arguments(
    attributes: &Attributes,
    data: StructTypeNode,
    discriminator: Option<InstructionArgumentNode>,
) -> Vec<InstructionArgumentNode> {
    let (before, after): (Vec<_>, Vec<_>) = attributes
        .get_all(ArgumentDirective::filter)
        .into_iter()
        .partition(|attr| !attr.after);

    let before = before
        .into_iter()
        .map(|attr| attr.argument.clone())
        .collect::<Vec<_>>();

    let after = after
        .into_iter()
        .map(|attr| attr.argument.clone())
        .collect::<Vec<_>>();

    let mut arguments: Vec<InstructionArgumentNode> = before
        .into_iter()
        .chain(Vec::<InstructionArgumentNode>::from(data))
        .chain(after)
        .collect();

    if let Some(discriminator) = discriminator {
        arguments.insert(0, discriminator);
    }

    arguments
}

fn parse_struct(
    korok: &codama_koroks::StructKorok,
) -> CodamaResult<(CamelCaseString, StructTypeNode)> {
    // Ensure we have a `DefinedTypeNode` to work with.
    if let Some(Node::DefinedType(node)) = &korok.node {
        // Ensure the data type is a struct.
        if let TypeNode::Struct(data) = node.r#type.clone() {
            return Ok((node.name.clone(), data));
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" struct could not be used as an Instruction because its type is not a `StructTypeNode`.",
        korok.ast.ident,
    );
    Err(korok.ast.error(message).into())
}

fn parse_enum_variant(
    korok: &codama_koroks::EnumVariantKorok,
    enum_name: &str,
) -> CodamaResult<(CamelCaseString, StructTypeNode)> {
    // Ensure we have a `Node`.
    if let Some(node) = &korok.node {
        // Ensure we have a `EnumVariantTypeNode`.
        if let Ok(node) = EnumVariantTypeNode::try_from(node.clone()) {
            match node {
                // Ensure we have a non-nested `StructTypeNode`.
                EnumVariantTypeNode::Struct(node) => {
                    if let NestedTypeNode::Value(data) = node.r#struct {
                        return Ok((node.name, data));
                    };
                }
                // Or an empty variant.
                EnumVariantTypeNode::Empty(node) => {
                    return Ok((node.name, StructTypeNode::new(vec![])))
                }
                _ => {}
            }
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" variant of the \"{enum_name}\" enum could not be used as an Instruction because we cannot get a `StructTypeNode` for it. This is likely because it is not using nammed fields.",
        korok.ast.ident
    );
    Err(korok.ast.error(message).into())
}
