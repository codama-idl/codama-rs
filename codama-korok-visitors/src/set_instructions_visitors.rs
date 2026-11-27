use crate::{
    enum_items::{
        data_field_identifier, parse_struct_data, parse_variant_data, EnumItemContext, ItemKind,
    },
    walk_enum, CombineTypesVisitor, KorokVisitor,
};
use codama_attributes::{
    AccountDirective, Attributes, CodamaAttribute, CodamaDirective, DefaultValueDirective,
    DiscriminatorDirective, DisplayDirective, EnumDiscriminatorDirective, FieldDirective,
    OptionalAccountStrategyDirective, ProgramDirective, RemainingAccountsDirective, Resolvable,
    SkipDirective, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    DefaultValueStrategy, Identifier, InjectedValueNode, InstructionAccountNode,
    InstructionDisplayNode, InstructionInputValueNode, InstructionNode, Node,
    OptionalAccountStrategy, ProgramNode, ProvidedNode, StructFieldTypeNode, StructTypeNode,
    TypeNode, ValueNode,
};
use codama_syn_helpers::extensions::ToTokensExtension;
use std::collections::HashMap;

pub struct SetInstructionsVisitor {
    combine_types: CombineTypesVisitor,
    /// The enum being visited, if any.
    r#enum: EnumItemContext,
    enum_optional_account_strategy: Option<OptionalAccountStrategy>,
}

impl Default for SetInstructionsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict()
                // An instruction provides the contextual default values of its data fields.
                .allowing_contextual_default_values()
                // Skip fields with the `account` codama directive, but reject a default value on them.
                .with_named_field(|korok, parent| {
                    if !korok.attributes.has(AccountDirective::filter) {
                        return CombineTypesVisitor::get_strict_named_field(korok, parent);
                    }
                    // An account field is not data, so a default value on it has no field to apply to.
                    let default_value = korok
                        .attributes
                        .iter()
                        .filter_map(CodamaAttribute::filter)
                        .find(|attribute| {
                            matches!(
                                attribute.directive.as_ref(),
                                CodamaDirective::DefaultValue(_)
                            )
                        })?;
                    let message = "A default value has no effect on an account field, \
                        use `account(..., default_value = ...)`";
                    Some(Err(default_value.ast.error(message).into()))
                }),
            r#enum: EnumItemContext::default(),
            enum_optional_account_strategy: None,
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
        let (name, data) = parse_struct_data(korok, ItemKind::Instruction)?;
        let (data, provides) = parse_data(&korok.attributes, &korok.fields, data, None)?;
        let instruction = InstructionNode {
            identifier: name,
            optional_account_strategy: parse_optional_account_strategy(&korok.attributes),
            accounts: parse_accounts(&korok.attributes, &korok.fields)?,
            data: Box::new(data),
            remaining_accounts: RemainingAccountsDirective::nodes(&korok.attributes),
            discriminators: DiscriminatorDirective::nodes(&korok.attributes),
            provides,
            display: parse_instruction_display(&korok.attributes),
            ..InstructionNode::default()
        };

        korok.node = Some(ProgramDirective::apply(
            &korok.attributes,
            instruction.into(),
        ));

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
        self.r#enum = EnumItemContext::new(korok.ast.ident.to_string(), enum_discriminator);
        self.enum_optional_account_strategy = parse_optional_account_strategy(&korok.attributes);
        walk_enum(self, korok)?;

        // Gather all instructions in a `ProgramNode`.
        let instructions = korok
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Instruction(instruction)) => Some(instruction.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        let node: Node = ProgramNode {
            instructions,
            ..ProgramNode::default()
        }
        .into();
        korok.node = Some(ProgramDirective::apply(&korok.attributes, node));

        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        let current_discriminator = self.r#enum.next_discriminator(korok)?;

        // Skip variants with #[codama(skip)] directive.
        if korok.attributes.has(SkipDirective::filter) {
            return Ok(());
        };

        let (name, data) = parse_variant_data(korok, &self.r#enum, ItemKind::Instruction)?;
        let discriminator = self.r#enum.discriminator_field(current_discriminator);

        let mut discriminators = DiscriminatorDirective::nodes(&korok.attributes);
        discriminators.insert(0, self.r#enum.discriminator_node().into());
        let (data, provides) =
            parse_data(&korok.attributes, &korok.fields, data, Some(discriminator))?;

        korok.node = Some(
            InstructionNode {
                identifier: name,
                // The variant-level directive wins over the enum-level default.
                optional_account_strategy: parse_optional_account_strategy(&korok.attributes)
                    .or(self.enum_optional_account_strategy),
                accounts: parse_accounts(&korok.attributes, &korok.fields)?,
                data: Box::new(data),
                remaining_accounts: RemainingAccountsDirective::nodes(&korok.attributes),
                discriminators,
                provides,
                display: parse_instruction_display(&korok.attributes),
                ..InstructionNode::default()
            }
            .into(),
        );

        Ok(())
    }
}

fn parse_optional_account_strategy(attributes: &Attributes) -> Option<OptionalAccountStrategy> {
    attributes
        .get_last(OptionalAccountStrategyDirective::filter)
        .map(|directive| directive.strategy)
}

fn parse_instruction_display(attributes: &Attributes) -> Option<InstructionDisplayNode> {
    // Display properties are host-specific. Field properties are intentionally left for
    // field and enum-variant display handling rather than interpreted as instruction metadata.
    attributes
        .get_all(DisplayDirective::filter)
        .into_iter()
        .fold(None, |display, directive| {
            if directive.intent.is_none() && directive.interpolated_intent.is_none() {
                return display;
            }

            let mut display = display.unwrap_or_default();
            if let Some(intent) = &directive.intent {
                display.intent = Some(intent.clone().into());
            }
            if let Some(interpolated_intent) = &directive.interpolated_intent {
                display.interpolated_intent = Some(interpolated_intent.clone().into());
            }
            Some(display)
        })
}

fn parse_accounts(
    attributes: &Attributes,
    fields: &[FieldKorok],
) -> CodamaResult<Vec<InstructionAccountNode>> {
    // Gather the accounts from the struct attributes.
    let accounts_from_struct_attributes = attributes
        .iter()
        .filter_map(AccountDirective::filter)
        .map(|attr| attr.to_instruction_account_node())
        .collect::<CodamaResult<Vec<_>>>()?;

    // Gather the accounts from the fields.
    let accounts_from_fields = fields
        .iter()
        .filter_map(|field| field.attributes.get_last(AccountDirective::filter))
        .map(|attr| attr.to_instruction_account_node())
        .collect::<CodamaResult<Vec<_>>>()?;

    // Concatenate the accounts.
    Ok(accounts_from_struct_attributes
        .into_iter()
        .chain(accounts_from_fields)
        .collect::<Vec<_>>())
}

fn parse_data(
    attributes: &Attributes,
    fields: &[FieldKorok],
    mut data: StructTypeNode,
    discriminator: Option<StructFieldTypeNode>,
) -> CodamaResult<(Option<TypeNode>, Vec<ProvidedNode>)> {
    // Fields are parsed in data order, so the `provides` list follows the same order.
    let mut provides = Vec::new();

    let defaults = get_data_field_defaults(attributes, fields)?;
    let fields_from_data = std::mem::take(&mut data.fields)
        .into_iter()
        .map(|data_field| match defaults.get(&data_field.identifier) {
            Some(default) if data_field.default_value.is_none() => {
                set_default_value(data_field, default, &mut provides)
            }
            _ => Ok(data_field),
        })
        .collect::<CodamaResult<Vec<_>>>()?;

    data.fields = discriminator.into_iter().chain(fields_from_data).collect();

    // An instruction without fields has no data.
    let data = (!data.fields.is_empty()).then(|| data.into());
    Ok((data, provides))
}

/// A default value written for an instruction data field.
struct DataFieldDefault<'a> {
    node: &'a Resolvable<InstructionInputValueNode>,
    strategy: Option<DefaultValueStrategy>,
}

/// Maps each data field identifier to the default written on its `field(...)` directive or Rust field.
/// Identifiers are unique in the data, so each field finds its own default.
fn get_data_field_defaults<'a>(
    attributes: &'a Attributes,
    fields: &'a [FieldKorok],
) -> CodamaResult<HashMap<Identifier, DataFieldDefault<'a>>> {
    let from_directives = attributes
        .get_all(FieldDirective::filter)
        .into_iter()
        .filter_map(|directive| {
            let default = DataFieldDefault {
                node: directive.default_value.as_ref()?,
                strategy: directive.default_value_strategy,
            };
            Some(Ok((directive.identifier.clone(), default)))
        });

    // A named account field is not data, but a tuple account item is both an account and a data item.
    let from_fields = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| {
            field.ast.ident.is_none() || !field.attributes.has(AccountDirective::filter)
        })
        .filter_map(|(index, field)| {
            let directive = field.attributes.get_last(DefaultValueDirective::filter)?;
            let default = DataFieldDefault {
                node: &directive.node,
                strategy: directive.default_value_strategy,
            };
            Some(data_field_identifier(fields, index).map(|identifier| (identifier, default)))
        });

    from_directives.chain(from_fields).collect()
}

/// Sets the default value of an instruction field.
/// A contextual value such as `payer` is not a data value, so the field injects it from the `provides` list.
fn set_default_value(
    field: StructFieldTypeNode,
    default: &DataFieldDefault,
    provides: &mut Vec<ProvidedNode>,
) -> CodamaResult<StructFieldTypeNode> {
    let node = default.node.try_resolved()?;
    let default_value = match ValueNode::try_from(node.clone()) {
        Ok(value) => value,
        Err(_) => {
            provides.push(ProvidedNode::new(field.identifier.clone(), node.clone()));
            InjectedValueNode::new(field.identifier.clone()).into()
        }
    };
    Ok(StructFieldTypeNode {
        default_value: Box::new(Some(default_value)),
        default_value_strategy: default.strategy,
        ..field
    })
}
