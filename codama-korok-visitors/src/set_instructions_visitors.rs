use crate::{
    assert_unique_field_identifiers, parse_field_display, CombineTypesVisitor, KorokVisitor,
};
use codama_attributes::{
    AccountDirective, Attributes, CodamaAttribute, CodamaDirective, DefaultValueDirective,
    DiscriminatorDirective, DisplayDirective, EnumDiscriminatorDirective, FieldDirective,
    OptionalAccountStrategyDirective, ProgramDirective, RemainingAccountsDirective, Resolvable,
    TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, EnumVariantTypeNode, FieldDiscriminatorNode,
    InjectedValueNode, InstructionAccountNode, InstructionDisplayNode, InstructionInputValueNode,
    InstructionNode, IntegerValueNode, Node, OptionalAccountStrategy, ProgramNode, ProvidedNode,
    StructFieldTypeNode, StructTypeNode, TypeNode, ValueNode,
};
use codama_syn_helpers::extensions::{ExprExtension, ToTokensExtension};
use std::collections::HashMap;

pub struct SetInstructionsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: String,
    enum_discriminator: EnumDiscriminatorDirective,
    enum_current_discriminator: usize,
    enum_optional_account_strategy: Option<OptionalAccountStrategy>,
}

impl Default for SetInstructionsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor {
                // An instruction provides the contextual default values of its data fields.
                allow_contextual_default_values: true,
                // Skip fields with the `account` codama directive, but reject a default value on them.
                get_nammed_field: |korok, parent| {
                    if !korok.attributes.has_codama_attribute("account") {
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
                },
                ..CombineTypesVisitor::strict()
            },
            enum_name: "".to_string(),
            enum_discriminator: EnumDiscriminatorDirective::default(),
            enum_current_discriminator: 0,
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
        let (name, data) = parse_struct(korok)?;
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
        self.enum_name = korok.ast.ident.to_string();
        self.enum_discriminator = enum_discriminator;
        self.enum_current_discriminator = 0;
        self.enum_optional_account_strategy = parse_optional_account_strategy(&korok.attributes);
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
        // Update current discriminator.
        let current_discriminator = match &korok.ast.discriminant {
            Some((_, expr)) => expr.as_unsigned_integer()?,
            _ => self.enum_current_discriminator,
        };
        self.enum_current_discriminator = current_discriminator + 1;

        // Skip variants with #[codama(skip)] directive.
        if korok.attributes.has_codama_attribute("skip") {
            return Ok(());
        };

        let (name, data) = parse_enum_variant(korok, &self.enum_name)?;
        let discriminator = StructFieldTypeNode {
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            default_value: Box::new(Some(
                IntegerValueNode::new(current_discriminator as u64).into(),
            )),
            ..StructFieldTypeNode::from(&self.enum_discriminator)
        };

        let parent = format!("variant `{}` of enum `{}`", korok.ast.ident, self.enum_name);
        let fields = std::iter::once(&discriminator).chain(&data.fields);
        assert_unique_field_identifiers(fields, &parent, korok.ast)?;

        let discriminator_node = FieldDiscriminatorNode::try_from(&self.enum_discriminator)?;
        let mut discriminators = DiscriminatorDirective::nodes(&korok.attributes);
        discriminators.insert(0, discriminator_node.into());
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

    let defaults = get_data_field_defaults(attributes, fields);
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
) -> HashMap<CamelCaseString, DataFieldDefault<'a>> {
    let from_directives = attributes
        .get_all(FieldDirective::filter)
        .into_iter()
        .filter_map(|directive| {
            let default = DataFieldDefault {
                node: directive.default_value.as_ref()?,
                strategy: directive.default_value_strategy,
            };
            Some((directive.name.clone(), default))
        });

    // A named account field is not data, but a tuple account item is both an account and a data item.
    let from_fields = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| {
            field.ast.ident.is_none() || !field.attributes.has_codama_attribute("account")
        })
        .filter_map(|(index, field)| {
            let directive = field.attributes.get_last(DefaultValueDirective::filter)?;
            let default = DataFieldDefault {
                node: &directive.node,
                strategy: directive.default_value_strategy,
            };
            Some((data_field_identifier(fields, index), default))
        });

    from_directives.chain(from_fields).collect()
}

/// The identifier of the data field that the Rust field at `index` gives.
/// An unnamed tuple item without `name = "..."` gets a synthetic identifier such as `arg0`.
fn data_field_identifier(fields: &[FieldKorok], index: usize) -> CamelCaseString {
    fields
        .get(index)
        .and_then(|field| field.name())
        .unwrap_or_else(|| format!("arg{}", index).into())
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

fn parse_struct(
    korok: &codama_koroks::StructKorok,
) -> CodamaResult<(CamelCaseString, StructTypeNode)> {
    // Ensure we have a `DefinedTypeNode` to work with.
    if let Some(Node::DefinedType(node)) = &korok.node {
        // Ensure the data type is a struct.
        if let TypeNode::Struct(data) = (*node.r#type).clone() {
            return Ok((node.identifier.clone(), data));
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
            match *node.data {
                Some(TypeNode::Struct(data)) => return Ok((node.identifier, data)),
                // Or an empty variant.
                None => return Ok((node.identifier, StructTypeNode::new(vec![]))),
                // Or a tuple variant — convert items to struct fields.
                Some(TypeNode::Tuple(tuple)) => {
                    let fields = tuple
                        .items
                        .into_iter()
                        .enumerate()
                        .map(|(i, item)| {
                            let name = data_field_identifier(&korok.fields, i);
                            let display = korok.fields.get(i).and_then(parse_field_display);
                            StructFieldTypeNode {
                                display,
                                ..StructFieldTypeNode::new(name, item)
                            }
                        })
                        .collect();
                    return Ok((node.identifier, StructTypeNode::new(fields)));
                }
                _ => {}
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
