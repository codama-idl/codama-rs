use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{
    DiscriminatorDirective, EnumDiscriminatorDirective, ProgramDirective, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, EnumVariantTypeNode, EventNode, FieldDiscriminatorNode,
    NestedTypeNode, NestedTypeNodeTrait, Node, NumberValueNode, ProgramNode, StructFieldTypeNode,
    StructTypeNode,
};
use codama_syn_helpers::extensions::*;

pub struct SetEventsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: String,
    enum_discriminator: EnumDiscriminatorDirective,
    enum_current_discriminator: usize,
}

impl Default for SetEventsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
            enum_name: "".to_string(),
            enum_discriminator: EnumDiscriminatorDirective::default(),
            enum_current_discriminator: 0,
        }
    }
}

impl SetEventsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetEventsVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaEvent` attribute.
        if !korok.attributes.has_codama_derive("CodamaEvent") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the struct, if it doesn't already exist.
        self.combine_types.visit_struct(korok)?;

        // Transform the defined type into an event node.
        let (name, data) = parse_struct(korok)?;

        let event = EventNode {
            discriminators: DiscriminatorDirective::nodes(&korok.attributes),
            ..EventNode::new(name, data)
        };

        korok.node = Some(ProgramDirective::apply(&korok.attributes, event.into()));

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the enum has the `CodamaEvents` attribute.
        if !korok.attributes.has_codama_derive("CodamaEvents") {
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

        // Transform each variant into an `EventNode`.
        self.enum_name = korok.ast.ident.to_string();
        self.enum_discriminator = enum_discriminator;
        self.enum_current_discriminator = 0;

        self.visit_children(korok)?;

        // Gather all events in the variants.
        let events = korok
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Event(event)) => Some(event.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        let node: Node = ProgramNode {
            events,
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

        let discriminator = StructFieldTypeNode {
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            default_value: Some(NumberValueNode::new(current_discriminator as u64).into()),
            ..StructFieldTypeNode::from(&self.enum_discriminator)
        };

        let discriminator_name = discriminator.name.clone();

        let (name, data) = parse_enum_variant(korok, &self.enum_name)?;

        let data = data.map_nested_type_node(|node| {
            let mut fields = node.fields;

            fields.insert(0, discriminator);

            StructTypeNode { fields }
        });

        let mut discriminators = DiscriminatorDirective::nodes(&korok.attributes);

        discriminators.insert(0, FieldDiscriminatorNode::new(discriminator_name, 0).into());

        let event = EventNode {
            discriminators,
            ..EventNode::new(name, data)
        };

        korok.node = Some(event.into());

        Ok(())
    }
}

fn parse_struct(
    korok: &codama_koroks::StructKorok,
) -> CodamaResult<(CamelCaseString, NestedTypeNode<StructTypeNode>)> {
    // Ensure we have a `DefinedTypeNode` to work with.
    if let Some(Node::DefinedType(node)) = &korok.node {
        // Ensure the data type is a struct.
        if let Ok(data) = NestedTypeNode::<StructTypeNode>::try_from(node.r#type.clone()) {
            return Ok((node.name.clone(), data));
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" struct could not be used as an Event because its type is not a `NestedTypeNode<StructTypeNode>`.",
        korok.ast.ident,
    );

    Err(korok.ast.error(message).into())
}

fn parse_enum_variant(
    korok: &codama_koroks::EnumVariantKorok,
    enum_name: &str,
) -> CodamaResult<(CamelCaseString, NestedTypeNode<StructTypeNode>)> {
    // Ensure we have a `Node`.
    if let Some(node) = &korok.node {
        // Ensure we have a `EnumVariantTypeNode`.
        if let Ok(node) = EnumVariantTypeNode::try_from(node.clone()) {
            match node {
                // Ensure we have a non-nested `StructTypeNode`.
                EnumVariantTypeNode::Struct(node) => {
                    return Ok((node.name, node.r#struct));
                }
                // Or an empty variant.
                EnumVariantTypeNode::Empty(node) => {
                    return Ok((node.name, StructTypeNode::new(vec![]).into()))
                }
                _ => {}
            }
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" variant of the \"{enum_name}\" enum could not be used as an Event because we cannot get a `StructTypeNode` for it. This is likely because it is not using named fields.",
        korok.ast.ident
    );

    Err(korok.ast.error(message).into())
}
