use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{
    AccountDirective, ArgumentDirective, Attributes, DefaultValueDirective, DiscriminatorDirective,
    EnumDiscriminatorDirective, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, EnumVariantTypeNode, FieldDiscriminatorNode,
    InstructionAccountNode, InstructionArgumentNode, InstructionNode, NestedTypeNode, Node,
    NumberValueNode, ProgramNode, StructFieldTypeNode, StructTypeNode, TypeNode,
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
                arguments: parse_arguments(&korok.attributes, &korok.fields, data, None),
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
                arguments: parse_arguments(
                    &korok.attributes,
                    &korok.fields,
                    data,
                    Some(discriminator),
                ),
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
    fields: &[FieldKorok],
    data: StructTypeNode,
    discriminator: Option<InstructionArgumentNode>,
) -> Vec<InstructionArgumentNode> {
    // Here we must reconcile the struct fields combined in the `CombineTypesVisitor`
    // with their original `FieldKoroks` to check for `default_value` directives
    // that would have been ignored on fields but are relevant for instruction arguments.
    //
    // For struct fields: match by name (accounts may be filtered, so positions don't align).
    // For tuple fields: match by index (no filtering, positions align directly).
    let arguments_from_data = Vec::<InstructionArgumentNode>::from(data)
        .into_iter()
        .enumerate()
        .map(|(i, argument)| {
            if argument.default_value.is_some() {
                return argument;
            }
            let field = fields
                .iter()
                .enumerate()
                .find_map(|(fi, field)| match field.name() {
                    Some(name) if name == argument.name => Some(field),
                    None if fi == i => Some(field),
                    _ => None,
                });
            let Some(field) = field else {
                return argument;
            };
            let Some(directive) = field.attributes.get_last(DefaultValueDirective::filter) else {
                return argument;
            };

            InstructionArgumentNode {
                default_value: Some(directive.node.clone()),
                default_value_strategy: directive.default_value_strategy,
                ..argument
            }
        });

    let (before, after): (Vec<_>, Vec<_>) = attributes
        .get_all(ArgumentDirective::filter)
        .into_iter()
        .partition(|attr| !attr.after);
    let before = before.into_iter().map(|attr| attr.argument.clone());
    let after = after.into_iter().map(|attr| attr.argument.clone());

    let mut arguments: Vec<InstructionArgumentNode> = before
        .into_iter()
        .chain(arguments_from_data)
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
                // Or a tuple variant — convert items to struct fields.
                // Use field.name() which returns #[codama(name = "...")] if provided,
                // otherwise fall back to synthetic names like "arg0", "arg1".
                EnumVariantTypeNode::Tuple(node) => {
                    if let NestedTypeNode::Value(tuple) = node.tuple {
                        let fields = tuple
                            .items
                            .into_iter()
                            .enumerate()
                            .map(|(i, item)| {
                                let name = korok
                                    .fields
                                    .get(i)
                                    .and_then(|f| f.name())
                                    .unwrap_or_else(|| format!("arg{}", i).into());
                                StructFieldTypeNode::new(name, item)
                            })
                            .collect();
                        return Ok((node.name, StructTypeNode::new(fields)));
                    };
                }
            }
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" variant of the \"{enum_name}\" enum could not be parsed as an instruction. \
        Ensure all variant fields resolve to valid type nodes.",
        korok.ast.ident
    );
    Err(korok.ast.error(message).into())
}
